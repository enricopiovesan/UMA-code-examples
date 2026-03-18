# Chapter 4 Labs

These labs turn the feature flag evaluator into a guided reader workflow instead of a one-off build-and-run example.

## Labs

- `lab1-country-match`: a direct country rule wins on the first condition.
- `lab2-rollout-match`: the deterministic rollout rule enables the flag for a sticky cohort.
- `lab3-default-fallback`: no rule matches, so the flag returns its default.
- `lab4-rule-language`: demonstrates `in`, numeric comparison, and logical operators in one scenario.

Use the Rust-first lab runner:

```bash
./scripts/run_lab.sh lab1-country-match
```

Use the TypeScript implementation directly:

```bash
./scripts/run_lab.sh --impl ts lab4-rule-language
```

Compare the Rust and TypeScript outputs across every lab:

```bash
./scripts/compare_impls.sh
```
