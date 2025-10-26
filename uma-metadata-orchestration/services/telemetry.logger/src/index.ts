import Ajv from "ajv";
import fs from "node:fs";

const ajv = new Ajv({ allErrors: true });

export function createValidator(schemaPath: string) {
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf-8"));
  const validate = ajv.compile(schema);
  return (evt: any) => {
    const ok = validate(evt);
    return ok
      ? { source: "telemetry.logger", event: "image.analyzed.v1", status: "passed" }
      : { source: "telemetry.logger", event: "image.analyzed.v1", status: "failed", reason: ajv.errorsText(validate.errors || []) };
  };
}
