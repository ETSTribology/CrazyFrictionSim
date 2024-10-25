use simulation::config::contact::Contact;
use serde_json;

#[test]
fn test_contact_deserialization() {
    let json_data = r#"
    {
        "friction_coefficient": 0.1,
        "enabled": true,
        "dhat": 0.001,
        "epsv": 0.01
    }"#;

    let contact: Contact = serde_json::from_str(json_data).unwrap();
    assert_eq!(contact.friction_coefficient, Some(0.1));
    assert_eq!(contact.enabled, Some(true));
    assert_eq!(contact.dhat, Some(0.001));
    assert_eq!(contact.epsv, Some(0.01));
}
