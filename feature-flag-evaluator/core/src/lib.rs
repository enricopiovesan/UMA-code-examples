//! Core logic for the feature flag evaluator.
//!
//! This crate implements a minimal, deterministic flag evaluation engine.  It is intentionally
//! dependency free and does not perform any I/O.  All logic is pure and functions can be
//! compiled to the `wasm32-wasi` target without modification.  The core supports a simple
//! expression language for flag rules and can be extended easily to support additional
//! operators.

use std::collections::HashMap;

/// A flag definition containing a unique key, a list of rules and a default value.
#[derive(Debug, Clone)]
pub struct Flag {
    /// Unique identifier for the flag (e.g. "paywall").
    pub key: String,
    /// Ordered list of rules.  The first rule whose condition evaluates to true
    /// determines the value returned by the evaluator.
    pub rules: Vec<Rule>,
    /// Default value returned when no rule matches or an error occurs.
    pub default: bool,
}

/// A single rule consisting of a condition and a resulting value.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Expression to evaluate.  The core supports the following forms:
    ///
    /// * `rollout(p)` – returns `true` if the deterministic rollout bucket for this flag
    ///   and the current `userId` is less than `p` where `0 <= p <= 1`.
    /// * Comparisons `==`, `!=`, `<`, `<=`, `>`, `>=` between context values and
    ///   literals (strings, numbers or booleans).
    /// * Membership `in`, for example `country in ('CA','US','MX')`.  Only string
    ///   membership is supported; numeric or boolean membership returns `false`.
    /// * Logical AND (`&&`) and OR (`||`) operators with short‑circuit evaluation.
    ///
    /// Unknown or malformed expressions cause the rule to be skipped.
    pub cond: String,
    /// The value to return when the condition evaluates to true.
    pub then_value: bool,
}

/// A value within a context.  Context values may be strings, numbers or booleans.
#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

/// A simple alias for the context map.  Keys are ASCII strings; values are s.
pub type Context = HashMap<String, Value>;

/// The result of flag evaluation.
#[derive(Debug, Clone)]
pub struct EvalResult {
    /// The flag key that was evaluated.
    pub key: String,
    /// Whether the flag is enabled.
    pub enabled: bool,
    /// Index of the matched rule, if any.   if no rule matched.
    pub matched_rule: Option<usize>,
}

/// Evaluate a flag against a context.
///
/// This function iterates through the flag's rules.  For each rule, it calls
/// [](fn.eval_rule_expr.html) to determine whether the rule
/// matches.  The first matching rule determines the return value.  If no
/// rule matches, the flag's default is used.  The function never panics.
pub fn eval_flag(flag: &Flag, ctx: &Context) -> EvalResult {
    if let Some((idx, val)) = eval_rules(flag, ctx) {
        return EvalResult {
            key: flag.key.clone(),
            enabled: val,
            matched_rule: Some(idx),
        };
    }
    EvalResult {
        key: flag.key.clone(),
        enabled: flag.default,
        matched_rule: None,
    }
}

/// Evaluate the rules of a flag.  Returns the index and value of the first
/// matching rule, or  if no rule matches.
pub fn eval_rules(flag: &Flag, ctx: &Context) -> Option<(usize, bool)> {
    for (i, rule) in flag.rules.iter().enumerate() {
        match eval_rule_expr(&flag.key, &rule.cond, ctx) {
            Ok(true) => return Some((i, rule.then_value)),
            Ok(false) => continue,
            Err(_) => continue, // malformed rule, skip
        }
    }
    None
}

/// Evaluate a single rule expression against the given context.
///
/// This function parses and evaluates boolean expressions consisting of
/// `rollout(p)`, comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`), membership
/// checks (`in`) and logical conjunction (`&&`) and disjunction (`||`).
/// Strings may be quoted with single or double quotes.  Lists for `in`
/// must be comma separated and enclosed in parentheses.  Unknown or
/// malformed expressions cause the rule to be skipped (the function
/// returns an `Err(())`).
pub fn eval_rule_expr(flag_key: &str, expr: &str, ctx: &Context) -> Result<bool, ()> {
    eval_expr(flag_key, expr.trim(), ctx)
}

/// Deterministic rollout function.  Given a flag key, user ID and probability
/// , returns  if the hash bucket falls below .  The implementation
/// uses a 32‑bit FNV‑1a hash to compute a value in [0, 1).
pub fn rollout(flag_key: &str, user_id: &str, p: f64) -> bool {
    let concatenated = format!("{}:{}", flag_key, user_id);
    // 32‑bit FNV‑1a parameters
    let mut hash: u32 = 0x811c9dc5;
    for byte in concatenated.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    // Map to [0,1) by dividing by 2^32
    let bucket = (hash as f64) / 4_294_967_296.0;
    bucket < p
}

/// Evaluate a boolean expression with logical operators, comparisons and built‑ins.
///
/// The grammar supported by this evaluator is a subset of the contract described in
/// the README:
///
/// ```
/// expr      := or_expr
/// or_expr   := and_expr { "||" and_expr }
/// and_expr  := cmp_expr { "&&" cmp_expr }
/// cmp_expr  := term { comp_op term }
/// comp_op   := "==" | "!=" | "<" | "<=" | ">" | ">=" | " in "
/// term      := ident | literal | rollout
/// ident     := unquoted identifier that resolves to a context value
/// literal   := string | number | boolean
/// rollout   := "rollout(" number ")"
/// ```
///
/// Strings may be quoted with single or double quotes.  Lists for the `in` operator
/// must be enclosed in parentheses and separated by commas, for example
/// `region in ('us','ca','eu')`.  Expressions that do not conform to this grammar
/// return an `Err(())` and cause their rule to be skipped.
fn eval_expr(flag_key: &str, s: &str, ctx: &Context) -> Result<bool, ()> {
    // Try OR level splitting
    if let Some(idx) = split_top_level(s, "||") {
        let left = &s[..idx];
        let right = &s[idx + 2..];
        let lval = eval_expr(flag_key, left.trim(), ctx)?;
        if lval {
            return Ok(true);
        }
        let rval = eval_expr(flag_key, right.trim(), ctx)?;
        return Ok(rval);
    }
    // Try AND level splitting
    if let Some(idx) = split_top_level(s, "&&") {
        let left = &s[..idx];
        let right = &s[idx + 2..];
        let lval = eval_expr(flag_key, left.trim(), ctx)?;
        if !lval {
            return Ok(false);
        }
        let rval = eval_expr(flag_key, right.trim(), ctx)?;
        return Ok(rval);
    }
    // Handle comparisons.  Check for the longest operators first to avoid partial matches.
    for op in [" in ", "<=", ">=", "==", "!=", "<", ">"] {
        if let Some(idx) = split_top_level(s, op) {
            let lhs = s[..idx].trim();
            let rhs = s[idx + op.len()..].trim();
            let left_value = parse_term_as_value(lhs, ctx, flag_key)?;
            // Special case for `in` where rhs should be a list of strings
            if op.trim() == "in" {
                return eval_in(left_value, rhs);
            }
            let right_value = parse_term_as_value(rhs, ctx, flag_key)?;
            return eval_comparison(left_value, op, right_value);
        }
    }
    // Otherwise parse single term as boolean
    match parse_term_as_value(s, ctx, flag_key)? {
        Value::Bool(b) => Ok(b),
        _ => Err(()),
    }
}

/// Split an expression at the first occurrence of `sep` that is not inside
/// quotes or parentheses.  Returns the index where `sep` starts.
fn split_top_level(s: &str, sep: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let sep_bytes = sep.as_bytes();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut paren_depth: i32 = 0;
    let mut i = 0;
    while i + sep_bytes.len() <= bytes.len() {
        let c = bytes[i] as char;
        // Toggle quote states
        if c == '\'' && !double_quote {
            single_quote = !single_quote;
            i += 1;
            continue;
        }
        if c == '"' && !single_quote {
            double_quote = !double_quote;
            i += 1;
            continue;
        }
        // Track parentheses depth
        if !single_quote && !double_quote {
            if c == '(' {
                paren_depth += 1;
            } else if c == ')' {
                paren_depth -= 1;
            }
            // Check for separator at top level
            if paren_depth == 0 && !single_quote && !double_quote {
                if bytes[i..].starts_with(sep_bytes) {
                    return Some(i);
                }
            }
        }
        i += 1;
    }
    None
}

/// Parse a term into a `Value`.  A term can be a literal, identifier or rollout call.
fn parse_term_as_value(term: &str, ctx: &Context, flag_key: &str) -> Result<Value, ()> {
    let t = term.trim();
    // rollout(p)
    if let Some(rest) = t.strip_prefix("rollout(") {
        if let Some(inner) = rest.strip_suffix(')') {
            let p: f64 = inner.trim().parse().map_err(|_| ())?;
            // Extract userId from context, default to empty string if missing or not a string
            let user_id = match ctx.get("userId") {
                Some(Value::Str(s)) => s.as_str(),
                _ => "",
            };
            return Ok(Value::Bool(rollout(flag_key, user_id, p)));
        }
    }
    // boolean literal
    if t.eq_ignore_ascii_case("true") {
        return Ok(Value::Bool(true));
    }
    if t.eq_ignore_ascii_case("false") {
        return Ok(Value::Bool(false));
    }
    // numeric literal
    if let Ok(n) = t.parse::<f64>() {
        return Ok(Value::Num(n));
    }
    // string literal in single or double quotes
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        let content = &t[1..t.len() - 1];
        return Ok(Value::Str(content.to_string()));
    }
    // identifier resolves from context
    if let Some(val) = ctx.get(t) {
        return Ok(val.clone());
    }
    // unknown identifier yields Null
    Ok(Value::Null)
}

/// Evaluate a comparison between two values using the given operator.
fn eval_comparison(left: Value, op: &str, right: Value) -> Result<bool, ()> {
    match (left, right) {
        (Value::Str(a), Value::Str(b)) => match op {
            "==" => Ok(a == b),
            "!=" => Ok(a != b),
            _ => Err(()),
        },
        (Value::Num(a), Value::Num(b)) => match op {
            "==" => Ok((a - b).abs() < std::f64::EPSILON),
            "!=" => Ok((a - b).abs() >= std::f64::EPSILON),
            "<" => Ok(a < b),
            "<=" => Ok(a <= b),
            ">" => Ok(a > b),
            ">=" => Ok(a >= b),
            _ => Err(()),
        },
        (Value::Bool(a), Value::Bool(b)) => match op {
            "==" => Ok(a == b),
            "!=" => Ok(a != b),
            _ => Err(()),
        },
        _ => Ok(false), // mismatched types always false for comparisons
    }
}

/// Evaluate the `in` operator: left must be a string and right must be a list of strings.
fn eval_in(left: Value, rhs: &str) -> Result<bool, ()> {
    match left {
        Value::Str(s) => {
            // Expect rhs to be something like ( 'A' , "B" , 'C' )
            let trimmed = rhs.trim();
            if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
                return Err(());
            }
            let inner = &trimmed[1..trimmed.len() - 1];
            for part in inner.split(',') {
                let token = part.trim();
                if (token.starts_with('"') && token.ends_with('"')) || (token.starts_with('\'') && token.ends_with('\'')) {
                    let content = &token[1..token.len() - 1];
                    if content == s {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build a context from a list of key/value pairs.
    fn ctx(pairs: &[(&str, &str)]) -> Context {
        let mut m = Context::new();
        for (k, v) in pairs {
            m.insert((*k).to_string(), Value::Str((*v).to_string()));
        }
        m
    }

    #[test]
    fn test_eval_flag_ca_rollout() {
        let flag = Flag {
            key: "paywall".to_string(),
            rules: vec![
                Rule { cond: "country == 'CA'".to_string(), then_value: true },
                Rule { cond: "rollout(0.20)".to_string(), then_value: true },
            ],
            default: false,
        };
        let ctx_map = ctx(&[("userId", "u123"), ("country", "CA")]);
        let res = eval_flag(&flag, &ctx_map);
        assert!(res.enabled);
        assert_eq!(res.matched_rule, Some(0));
    }

    #[test]
    fn test_eval_flag_rollout() {
        let flag = Flag {
            key: "paywall".to_string(),
            rules: vec![
                Rule { cond: "country == 'CA'".to_string(), then_value: true },
                Rule { cond: "rollout(0.20)".to_string(), then_value: true },
            ],
            default: false,
        };
        // Use a userId whose bucket is less than 0.20; "u20" yields about 0.0838
        let ctx_map = ctx(&[("userId", "u20"), ("country", "US")]);
        let res = eval_flag(&flag, &ctx_map);
        assert!(res.enabled);
        assert_eq!(res.matched_rule, Some(1));
    }

    #[test]
    fn test_eval_flag_default() {
        let flag = Flag {
            key: "paywall".to_string(),
            rules: vec![
                Rule { cond: "country == 'CA'".to_string(), then_value: true },
                Rule { cond: "rollout(0.00)".to_string(), then_value: true },
            ],
            default: false,
        };
        let ctx_map = ctx(&[("userId", "u999"), ("country", "US")]);
        let res = eval_flag(&flag, &ctx_map);
        assert!(!res.enabled);
        assert_eq!(res.matched_rule, None);
    }

    #[test]
    fn test_string_in_operator() {
        let flag = Flag {
            key: "region_test".to_string(),
            rules: vec![
                Rule { cond: "region in ('EU','APAC')".to_string(), then_value: true },
            ],
            default: false,
        };
        let ctx_map = ctx(&[("userId", "u1"), ("region", "EU")]);
        let res = eval_flag(&flag, &ctx_map);
        assert!(res.enabled);
        assert_eq!(res.matched_rule, Some(0));
        // region not in list should return default
        let ctx_map2 = ctx(&[("userId", "u2"), ("region", "NA")]);
        let res2 = eval_flag(&flag, &ctx_map2);
        assert!(!res2.enabled);
        assert_eq!(res2.matched_rule, None);
    }

    #[test]
    fn test_numeric_comparisons() {
        let flag = Flag {
            key: "version_test".to_string(),
            rules: vec![
                Rule { cond: "ver >= 2".to_string(), then_value: true },
                Rule { cond: "ver < 2".to_string(), then_value: false },
            ],
            default: false,
        };
        // context numeric values must be inserted as Value::Num to test numeric comparisons
        let mut ctx_map: Context = Context::new();
        ctx_map.insert("userId".to_string(), Value::Str("u3".to_string()));
        ctx_map.insert("ver".to_string(), Value::Num(3.0));
        let res = eval_flag(&flag, &ctx_map);
        assert!(res.enabled);
        assert_eq!(res.matched_rule, Some(0));
        // version less than 2
        let mut ctx_map2: Context = Context::new();
        ctx_map2.insert("userId".to_string(), Value::Str("u4".to_string()));
        ctx_map2.insert("ver".to_string(), Value::Num(1.0));
        let res2 = eval_flag(&flag, &ctx_map2);
        assert!(!res2.enabled);
        assert_eq!(res2.matched_rule, Some(1));
    }

    #[test]
    fn test_logical_operators() {
        let flag = Flag {
            key: "logic_test".to_string(),
            rules: vec![
                Rule { cond: "country == 'CA' && ver >= 2".to_string(), then_value: true },
                Rule { cond: "country == 'US' || country == 'MX'".to_string(), then_value: true },
            ],
            default: false,
        };
        // CA and ver >= 2 should match first rule
        let mut ctx_map: Context = Context::new();
        ctx_map.insert("userId".to_string(), Value::Str("u5".to_string()));
        ctx_map.insert("country".to_string(), Value::Str("CA".to_string()));
        ctx_map.insert("ver".to_string(), Value::Num(2.0));
        let res = eval_flag(&flag, &ctx_map);
        assert!(res.enabled);
        assert_eq!(res.matched_rule, Some(0));
        // US should match second rule via OR
        let mut ctx_map2: Context = Context::new();
        ctx_map2.insert("userId".to_string(), Value::Str("u6".to_string()));
        ctx_map2.insert("country".to_string(), Value::Str("US".to_string()));
        let res2 = eval_flag(&flag, &ctx_map2);
        assert!(res2.enabled);
        assert_eq!(res2.matched_rule, Some(1));
        // unknown country should not match any rule
        let mut ctx_map3: Context = Context::new();
        ctx_map3.insert("userId".to_string(), Value::Str("u7".to_string()));
        ctx_map3.insert("country".to_string(), Value::Str("BR".to_string()));
        ctx_map3.insert("ver".to_string(), Value::Num(5.0));
        let res3 = eval_flag(&flag, &ctx_map3);
        assert!(!res3.enabled);
        assert_eq!(res3.matched_rule, None);
    }
}