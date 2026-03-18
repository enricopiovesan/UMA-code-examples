
#[test]
fn contract_present() {
    let c = contract::Contract::load_from("../CONTRACT.json").expect("load");
    assert_eq!(c.service.name, "uma.image-analyzer");
}
