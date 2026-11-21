//! The suite's rule for a window at its minimum width: with a photo open and the Develop panel showing,
//! every dialog, Settings, the command palette and the update prompt keep every control inside the window
//! (the accessibility tree has the bounding box of each node). The guards are all on: no node without a
//! name, no glyph missing from the fonts.

use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::NodeT};

use crate::state::{RightPanel, ViewMode};
use crate::{LocalPhotoApp, Services};

fn harness() -> Harness<'static, LocalPhotoApp> {
    let mut app = LocalPhotoApp::new(localphoto_engine::Session::with_demo(), Services { png: None, ..Default::default() });
    app.ui.left_panel = true;
    app.ui.view = ViewMode::Detail;
    app.ui.right = RightPanel::Edit;
    let size = crate::window_place::MIN_SIZE;
    let mut h = Harness::builder().with_size(egui::vec2(size[0], size[1])).build_ui_state(
        |ui, app: &mut LocalPhotoApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(10);
    h
}

/// Every node whose box leaves the window, one line each.
fn outside(h: &Harness<'_, LocalPhotoApp>, place: &str, base: &std::collections::HashSet<String>) -> Vec<String> {
    let size = crate::window_place::MIN_SIZE;
    let mut out = Vec::new();
    for node in h.root().children_recursive() {
        let n = node.accesskit_node();
        if matches!(n.role(), Role::Window | Role::ScrollBar | Role::GenericContainer | Role::Unknown | Role::Group | Role::Pane) {
            continue;
        }
        let Some(b) = n.bounding_box() else { continue };
        // scrolled out of view: outside the box of a scroll area it sits in, so not on screen
        let mut up = n.parent();
        let mut hidden = false;
        while let Some(p) = up {
            if p.role() == Role::ScrollView
                && let Some(pb) = p.bounding_box()
                && (b.x1 <= pb.x0 || b.x0 >= pb.x1 || b.y1 <= pb.y0 || b.y0 >= pb.y1)
            {
                hidden = true;
                break;
            }
            up = p.parent();
        }
        if hidden {
            continue;
        }
        if b.width() <= 0.0 || b.height() <= 0.0 {
            continue;
        }
        let key = format!("{:?}|{}|{:.0},{:.0},{:.0},{:.0}", n.role(), n.label().unwrap_or_default(), b.x0, b.y0, b.x1, b.y1);
        // The library behind the dialog scrolls (side panels, filmstrip): its own nodes are not judged here,
        // only the top bar of the main screen and what the dialog itself adds.
        if place == "main" {
            if b.y1 > 60.0 {
                continue;
            }
        } else if base.contains(&key) {
            continue;
        }
        // Height is not judged: egui clamps windows to the screen and the rest of a tall dialog is content in
        // a scroll area, which the tree reports unclipped. Width is what a narrow window breaks.
        if b.x0 < -1.0 || b.y0 < -1.0 || b.x1 > f64::from(size[0]) + 1.0 {
            out.push(format!(
                "{place}: {:?} {:?} at ({:.0},{:.0})-({:.0},{:.0}) leaves the {}x{} window",
                n.role(),
                n.label().unwrap_or_default(),
                b.x0,
                b.y0,
                b.x1,
                b.y1,
                size[0],
                size[1]
            ));
        }
    }
    out
}

/// The nodes of the main screen, to tell what a dialog adds.
fn baseline(h: &Harness<'_, LocalPhotoApp>) -> std::collections::HashSet<String> {
    h.root()
        .children_recursive()
        .filter_map(|node| {
            let n = node.accesskit_node();
            let b = n.bounding_box()?;
            Some(format!("{:?}|{}|{:.0},{:.0},{:.0},{:.0}", n.role(), n.label().unwrap_or_default(), b.x0, b.y0, b.x1, b.y1))
        })
        .collect()
}

#[test]
fn every_dialog_fits_the_narrow_window() {
    let mut problems = Vec::new();
    // the main screen with the photo open and the Develop panel: the top bar (search, theme, gear) included
    let h = harness();
    let base = baseline(&h);
    problems.extend(outside(&h, "main", &base));

    // a face model file for the terms dialog
    let dir = std::env::temp_dir().join(format!("lp-narrow-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let model = dir.join("Mine.onnx");
    std::fs::write(&model, localphoto_faces::synthetic::tiny_embedder_model(512)).unwrap();

    for command in [
        "app.settings",
        "app.shortcuts",
        "app.whatsNew",
        "app.about",
        "app.systemInfo",
        "dialog.export",
        "dialog.copySettings",
        "dialog.pasteSettings",
        "dialog.createPreset",
        "dialog.rename",
        "dialog.newAlbum",
        "dialog.newFolder",
        "dialog.newSmartAlbum",
        "dialog.smartAlbum",
        "dialog.captureTime",
        "dialog.labelNames",
        "dialog.allMetadata",
        "dialog.saveMetadataPreset",
        "dialog.autoStack",
        "dialog.contactSheet",
        "dialog.slideshowExport",
        "dialog.cull",
        "dialog.manageMaskEffects",
        "dialog.faceModel",
    ] {
        let mut h = harness();
        let params = if command == "dialog.faceModel" { serde_json::json!({ "path": model.to_string_lossy() }) } else { serde_json::json!({}) };
        if let Err(e) = h.state_mut().run(command, params) {
            problems.push(format!("{command}: does not open ({e})"));
            continue;
        }
        h.run_steps(8);
        assert!(baseline(&h).difference(&base).count() > 0, "{command}: nothing new on screen, the dialog did not open");
        problems.extend(outside(&h, command, &base));
    }

    // the command palette
    let mut h = harness();
    h.state_mut().palette.open();
    h.run_steps(8);
    problems.extend(outside(&h, "palette", &base));

    // the update prompt and the manual check window, with a release name that is long on purpose
    let mut h = harness();
    h.state_mut().offer_update_for_test("Local Photo 99.0.0 with a release name long enough to need wrapping in a narrow window");
    h.run_steps(8);
    problems.extend(outside(&h, "update prompt", &base));

    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{} nodes off the window:\n{}", problems.len(), problems.join("\n"));
}
