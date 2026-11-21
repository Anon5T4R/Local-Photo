//! The suite's shell in the top bar: the Settings gear, the theme button (Automatic, Light, Dark) and the search
//! icon that opens the command palette, plus the menu items they share with the Edit and Help menus.

use std::time::Duration;

use serde_json::{Value, json};

use crate::headless::Headless;
use crate::menubar::{MenuNode, menu_bar};
use crate::state::{AppearanceMode, Dialog};
use crate::{LocalPhotoApp, Services};

const T: Duration = Duration::from_secs(20);

fn headless() -> Headless {
    let services = Services { png: None, ..Default::default() };
    let app = LocalPhotoApp::new(localphoto_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1400.0, 900.0], 1.0);
    h.settle(Duration::from_secs(60));
    h
}

fn click(h: &mut Headless, id: &str) {
    assert_eq!(h.request("ui.clickWidget", json!({"id": id}), T)["ok"], true, "{id}");
    h.step();
    h.step();
}

fn widget(h: &Headless, id: &str) -> egui::Rect {
    h.app.widgets.iter().find(|(w, _)| w == id).map(|(_, r)| *r).unwrap_or_else(|| panic!("no widget {id}"))
}

/// The items of one top-level menu, flattened (submenus included).
fn items(h: &Headless, menu: &str) -> Vec<(String, String, bool)> {
    fn walk(nodes: &[MenuNode], out: &mut Vec<(String, String, bool)>) {
        for node in nodes {
            match node {
                MenuNode::Item { id, label, enabled, .. } => out.push((id.clone(), label.clone(), *enabled)),
                MenuNode::Submenu { children, .. } => walk(children, out),
                MenuNode::Separator => {}
            }
        }
    }
    let mut out = Vec::new();
    for (title, nodes) in menu_bar(&h.app) {
        if title == menu {
            walk(&nodes, &mut out);
        }
    }
    out
}

#[test]
fn the_gear_opens_settings_and_so_does_the_edit_menu() {
    let mut h = headless();
    assert!(h.app.ui.dialog.is_none());
    click(&mut h, "icon:gear");
    assert!(matches!(h.app.ui.dialog, Some(Dialog::Settings { .. })), "the gear opens Settings");
    h.app.ui.dialog = None;
    assert!(items(&h, "Edit").iter().any(|(id, label, enabled)| id == "app.settings" && label == "Settings…" && *enabled));
    crate::menubar::run_item(&mut h.app, "app.settings", Value::Null).unwrap();
    assert!(matches!(h.app.ui.dialog, Some(Dialog::Settings { .. })), "Edit > Settings… opens the same window");
}

#[test]
fn the_bar_orders_gear_theme_search_from_the_right_edge() {
    let h = headless();
    let (gear, theme, search) = (widget(&h, "icon:gear"), widget(&h, "icon:appearance"), widget(&h, "icon:palette"));
    assert!(gear.left() > theme.right() && theme.left() > search.right(), "{gear:?} {theme:?} {search:?}");
    for other in ["icon:help", "icon:share", "icon:import", "icon:bell"] {
        assert!(widget(&h, other).right() <= search.left(), "{other} stays left of the suite's buttons");
    }
}

#[test]
fn the_theme_menu_and_the_button_take_the_three_modes() {
    let mut h = headless();
    let view = items(&h, "View");
    for label in ["Automatic", "Light Mode", "Dark Mode"] {
        assert!(view.iter().any(|(id, l, _)| id == "view.appearance" && l == label), "View > Appearance lists {label}");
    }
    for (param, mode) in [("light", AppearanceMode::Light), ("auto", AppearanceMode::Auto), ("dark", AppearanceMode::Dark)] {
        crate::menubar::run_item(&mut h.app, "view.appearance", json!({"mode": param})).unwrap();
        assert_eq!(h.app.ui.settings.appearance_mode, mode);
    }
    // the button steps Automatic, Light, Dark, like the suite's theme button
    for mode in [AppearanceMode::Auto, AppearanceMode::Light, AppearanceMode::Dark] {
        click(&mut h, "icon:appearance");
        assert_eq!(h.app.ui.settings.appearance_mode, mode);
    }
}

#[test]
fn language_is_chosen_in_settings_and_no_longer_in_the_edit_menu() {
    let mut h = headless();
    assert!(!items(&h, "Edit").iter().any(|(id, ..)| id.starts_with("app.language.")), "no Language submenu in Edit");
    // the commands stay for the control channel, and Settings > General has the picker
    h.request("engine.execute", json!({"command": "app.language.german"}), T);
    assert_eq!(h.app.ui.language, crate::i18n::Locale::De);
    h.request("engine.execute", json!({"command": "app.language.english"}), T);
    assert_eq!(h.app.ui.language, crate::i18n::Locale::En);
}

#[test]
fn help_has_updates_and_about_and_file_ends_with_exit() {
    let h = headless();
    let help = items(&h, "Help");
    let position = |label: &str| help.iter().position(|(_, l, _)| l == label).unwrap_or_else(|| panic!("Help has no {label}"));
    assert!(position("Check for Updates…") < position("About Local Photo"));
    let file = items(&h, "File");
    assert_eq!(file.last().map(|(id, label, _)| (id.as_str(), label.as_str())), Some(("app.quit", "Exit")));
}

#[test]
fn the_search_icon_opens_the_palette_which_lists_the_menus_and_runs_a_command() {
    let mut h = headless();
    assert!(!h.app.palette.is_open());
    click(&mut h, "icon:palette");
    assert!(h.app.palette.is_open(), "the search icon opens the palette");

    let listed = crate::shell::commands(&h.app, false);
    for group in ["File", "Edit", "View", "Photo", "Window", "Help"] {
        assert!(listed.iter().any(|(command, ..)| command.group.starts_with(group)), "{group} has commands in the palette");
    }
    let find = |label: &str| {
        listed
            .iter()
            .find(|(command, ..)| command.label == label)
            .map(|(command, ..)| command.clone())
            .unwrap_or_else(|| panic!("{label} is not in the palette"))
    };
    assert!(find("Settings…").enabled && find("Exit").enabled && find("Check for Updates…").enabled);
    assert_eq!(find("Automatic").group, "View › Appearance");
    // a disabled menu item is listed and stays disabled (Redo with nothing to redo)
    assert!(!find("Redo").enabled);

    // typing narrows the list and Enter runs the first match through the menu's own code
    assert_eq!(h.request("ui.text", json!({"text": "settings"}), T)["ok"], true);
    h.step();
    assert_eq!(h.request("ui.key", json!({"key": "Enter"}), T)["ok"], true);
    h.step();
    h.step();
    assert!(!h.app.palette.is_open(), "the palette closes after running");
    assert!(matches!(h.app.ui.dialog, Some(Dialog::Settings { .. })), "Settings… ran");
}

#[test]
fn escape_closes_the_palette_and_its_keys_stay_off_the_photos() {
    let mut h = headless();
    let before = h.app.session.active();
    click(&mut h, "icon:palette");
    assert!(h.app.palette.is_open());
    // an arrow key moves the palette's selection, not the grid's
    assert_eq!(h.request("ui.key", json!({"key": "ArrowRight"}), T)["ok"], true);
    h.step();
    assert_eq!(h.app.session.active(), before);
    assert_eq!(h.request("ui.key", json!({"key": "Escape"}), T)["ok"], true);
    h.step();
    assert!(!h.app.palette.is_open());
}

#[test]
fn ctrl_k_stays_add_keywords_and_does_not_open_the_palette() {
    let mut h = headless();
    assert_eq!(h.request("ui.key", json!({"key": "K", "ctrl": true}), T)["ok"], true);
    h.step();
    h.step();
    assert!(!h.app.palette.is_open(), "Ctrl+K is Add Keywords in Local Photo; the palette opens from the search icon only");
}

/// Writes `image` as a PNG.
fn save_png(image: &egui::ColorImage, path: &std::path::Path) {
    let data: Vec<[u8; 4]> = image.pixels.iter().map(|c| c.to_array()).collect();
    let rgba = localphoto_raster::Rgba8 { width: image.size[0], height: image.size[1], data };
    let bytes = localphoto_codecs::encode_png(&localphoto_codecs::EncodeImage::rgba8(&rgba), &localphoto_codecs::EncodeMeta::default()).unwrap();
    std::fs::write(path, bytes).unwrap();
}

/// Visual QA (ignored): the window with the suite's top bar in both themes, Settings and the palette open. The PNGs
/// land in `target/qa-casca/` (the CPU rasterizer needs no GPU).
#[test]
#[ignore = "visual QA: renders the window with the suite's shell in both themes"]
fn casca_shots() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/qa-casca");
    std::fs::create_dir_all(&directory).unwrap();
    for (name, mode) in [("light", "light"), ("dark", "dark")] {
        for what in ["barra", "configuracoes", "paleta"] {
            let mut h = headless();
            h.app.run("view.appearance", json!({"mode": mode})).unwrap();
            match what {
                "configuracoes" => {
                    h.app.run("app.settings", json!({})).unwrap();
                }
                "paleta" => {
                    h.app.palette.open();
                    h.step();
                    h.request("ui.text", json!({"text": "set"}), T);
                }
                _ => {}
            }
            h.step();
            h.step();
            let image = h.snapshot(Duration::from_secs(30));
            save_png(&image, &directory.join(format!("{name}-{what}.png")));
        }
    }
}

/// Visual QA (ignored): the main screen with a photo open and the Develop panel, the palette, Settings, every dialog
/// and the update prompt, at the minimum window and at 1100 wide, in both themes. The PNGs land in
/// `target/qa-guardas/`.
#[test]
#[ignore = "visual QA: renders the app at two widths in both themes"]
fn guardas_shots() {
    use crate::state::{RightPanel, ViewMode};
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/qa-guardas");
    std::fs::create_dir_all(&directory).unwrap();
    let model_dir = std::env::temp_dir().join(format!("lp-shots-{}", std::process::id()));
    std::fs::create_dir_all(&model_dir).unwrap();
    let model = model_dir.join("Mine.onnx");
    std::fs::write(&model, localphoto_faces::synthetic::tiny_embedder_model(512)).unwrap();
    let scenes = [
        "main", "palette", "app.settings", "app.shortcuts", "app.whatsNew", "app.about", "app.systemInfo", "dialog.export", "dialog.copySettings",
        "dialog.createPreset", "dialog.rename", "dialog.newAlbum", "dialog.newSmartAlbum", "dialog.captureTime", "dialog.labelNames",
        "dialog.allMetadata", "dialog.autoStack", "dialog.contactSheet", "dialog.slideshowExport", "dialog.cull", "dialog.faceModel", "update",
    ];
    for (wname, size) in [("min", crate::window_place::MIN_SIZE), ("1100", [1100.0, 700.0])] {
        for mode in ["light", "dark"] {
            for scene in scenes {
                let services = Services { png: None, ..Default::default() };
                let mut app = LocalPhotoApp::new(localphoto_engine::Session::with_demo(), services);
                app.ui.left_panel = true;
                app.ui.view = ViewMode::Detail;
                app.ui.right = RightPanel::Edit;
                let mut h = Headless::new(app, size, 1.0);
                h.settle(Duration::from_secs(60));
                h.app.run("view.appearance", json!({"mode": mode})).unwrap();
                match scene {
                    "main" => {}
                    "palette" => {
                        h.app.palette.open();
                        h.step();
                    }
                    "update" => h.app.offer_update_for_test("Local Photo 99.0.0 with a release name long enough to need wrapping in a narrow window"),
                    "dialog.faceModel" => {
                        h.app.run(scene, json!({"path": model.to_string_lossy()})).unwrap();
                    }
                    command => {
                        h.app.run(command, json!({})).unwrap();
                    }
                }
                for _ in 0..4 {
                    h.step();
                }
                let image = h.snapshot(Duration::from_secs(30));
                save_png(&image, &directory.join(format!("{wname}-{mode}-{scene}.png")));
            }
        }
    }
}
