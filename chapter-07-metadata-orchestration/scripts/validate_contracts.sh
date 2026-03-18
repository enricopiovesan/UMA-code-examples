#!/usr/bin/env bash
set -euo pipefail
echo "Validating contracts exist and schemas are JSON parseable"
test -f contracts/image.tagger.contract.yaml
test -f contracts/telemetry.logger.contract.yaml
test -f contracts/edge.cache.contract.yaml
test -f contracts/ai.workflow.orchestrator.contract.yaml
jq . contracts/schemas/image.analyzed.v1.json >/dev/null
jq . contracts/schemas/telemetry.validation.v1.json >/dev/null
jq . contracts/schemas/uma.cloudevents.ext.json >/dev/null
jq . contracts/schemas/policy.standard.v1.json >/dev/null
echo "OK"
