
#[test]
fn contract_validates_against_schema() {
    use jsonschema::{JSONSchema, Draft};
    let contract_path = "../../CONTRACT.json";
    let schema_path = "../../../schemas/uma-contract.schema.json";
    let c: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(contract_path).unwrap()).unwrap();
    let s: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(schema_path).unwrap()).unwrap();
    let compiled = JSONSchema::options().with_draft(Draft::Draft7).compile(&s).unwrap();
    if let Err(errs) = compiled.validate(&c) {
        let v: Vec<String> = errs.map(|e| e.to_string()).collect();
        panic!("Contract failed schema validation: {:?}", v);
    }
}
