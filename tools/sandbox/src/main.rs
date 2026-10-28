use horizon_mods::{Mods, UiWidget};
use std::{env, fs, path::Path};

fn rows(mods: &Mods) -> Vec<String> {
    mods.screen(0)
        .unwrap()
        .widgets
        .iter()
        .find_map(|w| match w {
            UiWidget::List(l) => Some(l.rows.clone()),
            _ => None,
        })
        .unwrap()
}
fn text(mods: &Mods) -> String {
    mods.screen(0)
        .unwrap()
        .widgets
        .iter()
        .filter_map(|w| match w {
            UiWidget::Text(t) => Some(t.text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}
fn main() {
    let artifact = env::args().nth(1).expect("pass mod_menu.hmod");
    let dir = env::temp_dir().join(format!("mod-menu-sandbox-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let path = dir.join("mod_menu.hmod");
    fs::copy(Path::new(&artifact), &path).unwrap();
    let storage = dir.join(".horizon-mod-data/mod_menu");
    fs::create_dir_all(&storage).unwrap();
    let mut entries: Vec<_> = (0..25).map(|i| serde_json::json!({"id":format!("mod_{i:02}"),"name":format!("Mod {i:02}"),"version":"1.0","kind":"Sandboxed","enabled":false})).collect();
    entries.push(serde_json::json!({"id":"veinminer","name":"Veinminer","version":"0.3.0","kind":"Native","enabled":true}));
    let inventory = serde_json::json!({"schema":1,"mods":entries});
    fs::write(storage.join("inventory.json"), inventory.to_string()).unwrap();
    let mut mods = Mods::load(&[path.clone()], |_| true, |_| false);
    assert!(mods.is_running(), "{:?}", mods.entries()[0].status);
    assert!(mods.command("modmenu"));
    assert_eq!(rows(&mods).len(), 20);
    assert!(text(&mods).contains("26 installed | 1 enabled"));
    mods.ui_select(0, 4, 0);
    assert_eq!(rows(&mods).len(), 6);
    assert!(
        rows(&mods)
            .iter()
            .any(|r| r.contains("Veinminer") && r.contains("[On]"))
    );
    mods.ui_select(0, 2, 0);
    assert_eq!(rows(&mods).len(), 1);
    mods.ui_select(0, 1, 0);
    assert!(text(&mods).contains("ID: veinminer"));
    assert!(text(&mods).contains("Type: Native"));
    mods.ui_select(0, 2, 0);
    assert_eq!(rows(&mods).len(), 20);
    mods.ui_select(0, 2, 0);
    assert!(mods.command("modmenu VEIN"));
    assert_eq!(rows(&mods).len(), 1);
    assert!(rows(&mods)[0].contains("Veinminer"));
    fs::write(
        storage.join("inventory.json"),
        b"{\"schema\":2,\"mods\":[]}",
    )
    .unwrap();
    mods.ui_select(0, 5, 0);
    assert!(text(&mods).contains("newer Mod Menu"));
    fs::write(storage.join("inventory.json"), inventory.to_string()).unwrap();
    mods.key("key.keyboard.m", true);
    mods.tick(1);
    assert_eq!(rows(&mods).len(), 1);
    mods.ui_settings_opened(0);
    assert!(!mods.settings_widgets(0).is_empty());
    fs::remove_file(storage.join("inventory.json")).unwrap();
    assert!(mods.command("modmenu"));
    assert!(text(&mods).contains("No inventory feed"));
    drop(mods);
    fs::remove_dir_all(dir).unwrap();
    println!(
        "PASS: actual .hmod sandbox loads, lists native/disabled mods, pages, filters, searches, selects, refreshes, opens with M/settings, reports missing/incompatible feed"
    );
}
