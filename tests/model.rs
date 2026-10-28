use horizon_mod_menu::model::{self, Filter, InstalledMod};

fn installed(id: &str, enabled: bool) -> InstalledMod {
    InstalledMod {
        id: id.into(),
        name: id.into(),
        version: "1.0".into(),
        kind: "Native".into(),
        enabled,
        problem: None,
    }
}
#[test]
fn inventory_preserves_disabled_and_native_mods_and_sorts_names() {
    let inventory = model::parse(br#"{"schema":1,"mods":[{"id":"z","name":"Zulu","version":"1","kind":"Native","enabled":false},{"id":"a","name":"Alpha","version":"2","kind":"Sandboxed","enabled":true}]}"#).unwrap();
    assert_eq!(inventory.mods[0].id, "a");
    assert!(!inventory.mods[1].enabled);
    assert_eq!(inventory.mods[1].kind, "Native");
}
#[test]
fn incompatible_or_broken_feeds_do_not_look_like_an_empty_installation() {
    assert!(model::parse(b"{}").is_err());
    assert!(model::parse(br#"{"schema":2,"mods":[]}"#).is_err());
    assert!(model::parse(b"not json").is_err());
    assert!(model::parse(&vec![b' '; (1 << 20) + 1]).is_err());
}
#[test]
fn filters_search_and_page_boundaries_keep_all_installed_mods_reachable() {
    let mods: Vec<_> = (0..41)
        .map(|i| installed(&format!("mod_{i:02}"), i % 2 == 0))
        .collect();
    assert_eq!(model::page_count(mods.len()), 3);
    assert_eq!(model::matching(&mods, Filter::Enabled, "").len(), 21);
    assert_eq!(model::matching(&mods, Filter::Disabled, "").len(), 20);
    assert_eq!(
        model::matching(&mods, Filter::All, "MOD_40")[0].id,
        "mod_40"
    );
    assert_eq!(model::matching(&mods, Filter::All, "native").len(), 41);
    assert_eq!(model::page_count(0), 1);
    assert_eq!(Filter::All.next().next().next(), Filter::All);
}
#[test]
fn mod_names_cannot_inject_legacy_colors_or_multiline_rows() {
    assert_eq!(model::clean("One\nTwo§c"), "OneTwoc");
    assert_eq!(model::clean(&"界".repeat(500)).chars().count(), 180);
    let mut broken = installed("broken", true);
    broken.problem = Some("Invalid package".into());
    assert!(broken.row().ends_with("[Invalid]"));
}
