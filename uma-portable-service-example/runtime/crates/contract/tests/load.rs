
#[test]
fn loads_contract() {
    let path = "../../CONTRACT.json";
    let c = contract::Contract::load_from(path).expect("load contract");
    assert_eq!(c.service.name, "uma.image-analyzer");
    assert_eq!(c.service.version, "1.0.0");
    assert!(c.events.iter().any(|e| e.name == "image.analyzed"));
}
