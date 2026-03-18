function splitTopLevel(source, separator) {
  let single = false;
  let double = false;
  let depth = 0;

  for (let i = 0; i <= source.length - separator.length; i += 1) {
    const char = source[i];
    if (char === "'" && !double) {
      single = !single;
      continue;
    }
    if (char === '"' && !single) {
      double = !double;
      continue;
    }
    if (!single && !double) {
      if (char === '(') {
        depth += 1;
      } else if (char === ')') {
        depth -= 1;
      }
      if (depth === 0 && source.slice(i, i + separator.length) === separator) {
        return i;
      }
    }
  }

  return -1;
}

export function rollout(flagKey, userId, probability) {
  const input = `${flagKey}:${userId}`;
  let hash = 0x811c9dc5;
  for (const byte of Buffer.from(input, 'utf8')) {
    hash ^= byte;
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash / 4294967296 < probability;
}

function parseTerm(term, context, flagKey) {
  const trimmed = term.trim();
  if (trimmed.startsWith('rollout(') && trimmed.endsWith(')')) {
    const probability = Number(trimmed.slice('rollout('.length, -1).trim());
    if (Number.isNaN(probability)) {
      throw new Error('invalid rollout');
    }
    return rollout(flagKey, typeof context.userId === 'string' ? context.userId : '', probability);
  }
  if (trimmed === 'true') return true;
  if (trimmed === 'false') return false;
  if ((trimmed.startsWith("'") && trimmed.endsWith("'")) || (trimmed.startsWith('"') && trimmed.endsWith('"'))) {
    return trimmed.slice(1, -1);
  }
  if (!Number.isNaN(Number(trimmed)) && trimmed !== '') {
    return Number(trimmed);
  }
  return Object.hasOwn(context, trimmed) ? context[trimmed] : null;
}

function evalIn(left, rhs) {
  if (typeof left !== 'string') {
    return false;
  }
  const trimmed = rhs.trim();
  if (!trimmed.startsWith('(') || !trimmed.endsWith(')')) {
    throw new Error('invalid in');
  }
  return trimmed
    .slice(1, -1)
    .split(',')
    .map((token) => token.trim())
    .filter((token) => (token.startsWith("'") && token.endsWith("'")) || (token.startsWith('"') && token.endsWith('"')))
    .map((token) => token.slice(1, -1))
    .includes(left);
}

function evalComparison(left, op, right) {
  if (typeof left === 'string' && typeof right === 'string') {
    if (op === '==') return left === right;
    if (op === '!=') return left !== right;
    throw new Error('invalid string comparison');
  }
  if (typeof left === 'number' && typeof right === 'number') {
    if (op === '==') return Math.abs(left - right) < Number.EPSILON;
    if (op === '!=') return Math.abs(left - right) >= Number.EPSILON;
    if (op === '<') return left < right;
    if (op === '<=') return left <= right;
    if (op === '>') return left > right;
    if (op === '>=') return left >= right;
    throw new Error('invalid numeric comparison');
  }
  if (typeof left === 'boolean' && typeof right === 'boolean') {
    if (op === '==') return left === right;
    if (op === '!=') return left !== right;
    throw new Error('invalid boolean comparison');
  }
  return false;
}

export function evalRuleExpr(flagKey, expr, context) {
  return evalExpr(flagKey, expr.trim(), context);
}

function evalExpr(flagKey, source, context) {
  const orIndex = splitTopLevel(source, '||');
  if (orIndex >= 0) {
    return evalExpr(flagKey, source.slice(0, orIndex).trim(), context) ||
      evalExpr(flagKey, source.slice(orIndex + 2).trim(), context);
  }
  const andIndex = splitTopLevel(source, '&&');
  if (andIndex >= 0) {
    return evalExpr(flagKey, source.slice(0, andIndex).trim(), context) &&
      evalExpr(flagKey, source.slice(andIndex + 2).trim(), context);
  }

  for (const op of [' in ', '<=', '>=', '==', '!=', '<', '>']) {
    const idx = splitTopLevel(source, op);
    if (idx >= 0) {
      const left = parseTerm(source.slice(0, idx), context, flagKey);
      const rightExpr = source.slice(idx + op.length);
      if (op.trim() === 'in') {
        return evalIn(left, rightExpr);
      }
      const right = parseTerm(rightExpr, context, flagKey);
      return evalComparison(left, op, right);
    }
  }

  const value = parseTerm(source, context, flagKey);
  if (typeof value !== 'boolean') {
    throw new Error('non boolean');
  }
  return value;
}

export function evalFlag(input) {
  const { flag, context } = input;
  for (let index = 0; index < flag.rules.length; index += 1) {
    const rule = flag.rules[index];
    try {
      if (evalRuleExpr(flag.key, rule.if, context)) {
        return {
          key: flag.key,
          enabled: rule.then,
          matchedRule: index,
        };
      }
    } catch {
      continue;
    }
  }
  return {
    key: flag.key,
    enabled: flag.default ?? false,
    matchedRule: null,
  };
}
