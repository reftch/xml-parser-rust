use crate::config::settings::Settings;

#[test]
fn test_settings() {
    let settings = Settings::new();
    assert_eq!(settings.settings.len(), 3);
    assert_eq!(settings.settings.get("debug").unwrap() == "true", false);
    assert_eq!(settings.settings.get("sources").unwrap(), "xml");
    assert_eq!(settings.settings.get("destination").unwrap(), "markdown/");
}
