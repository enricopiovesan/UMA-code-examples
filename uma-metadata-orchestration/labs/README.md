# Failure path labs

## 1. Schema mismatch
Edit contracts/schemas/image.analyzed.v1.json and add a new required field, then run:
```
node tools/validator.js
./scripts/run_cloud.sh
```
Expected: validator or runtime fails with validation.failed.

## 2. Policy violation
Set ai.model.evaluator placement to include browser in ai.model.evaluator.contract.yaml and run:
```
POLICY_FAIL_MODE=closed ./scripts/run_cloud.sh
```
Expected: runner exits with [error] policy.violation.

## 3. Latency drift
Set an artificial sleep in services/image.tagger/src/main.rs, recompile, run:
```
./scripts/build_all.sh
./scripts/run_cloud.sh
node tools/validator.js audit
```
Expected: drift.latency warns above target.
