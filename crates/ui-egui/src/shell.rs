//! The suite's shell in the top bar: the Settings gear, the theme button and the command palette (the search
//! icon), drawn with the suite's `craft-icons` and `craft-palette`. What each does is the app's own command
//! (`app.settings`, `view.appearance`, the menu items), so the buttons, the menus and the control channel agree.

use craft_icons::{Icon, ThemeMode};
use craft_palette::{Command, Labels, Palette};
use egui::{Rect, Ui, UiBuilder, pos2, vec2};
use serde_json::{Value, json};

use crate::LocalPhotoApp;
use crate::menubar::{MenuNode, display_item_label, menu_bar, run_item, shortcut_text};
use crate::state::AppearanceMode;
use crate::widgets::{announce, register};

/// The side of one button's box in the bar, the gap between two of them and the first one's gap to the bar's edge
/// (the bar's own icons use the same 4, which keeps the window's buttons as far from both edges as each other).
const BOX: f32 = 28.0;
const GAP: f32 = 6.0;
const EDGE: f32 = 4.0;
/// How far the three buttons reach from the bar's right edge, edge gap included.
pub const REACH: f32 = EDGE + 3.0 * BOX + 2.0 * GAP;

/// The suite's mode for Local Photo's appearance setting (the ids on disk are unchanged: auto, dark, light).
pub fn theme_mode(mode: AppearanceMode) -> ThemeMode {
    match mode {
        AppearanceMode::Auto => ThemeMode::System,
        AppearanceMode::Light => ThemeMode::Light,
        AppearanceMode::Dark => ThemeMode::Dark,
    }
}

/// The parameter `view.appearance` takes for a mode.
fn mode_param(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::System => "auto",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    }
}

/// The mode's name as the menus and Settings say it (the catalog key; "Light" alone is the Light panel's).
pub fn mode_label(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::System => "Automatic",
        ThemeMode::Light => "Light Mode",
        ThemeMode::Dark => "Dark Mode",
    }
}

/// "Theme: Automatic": the theme button's tooltip, and what the palette names the three modes.
fn theme_text(mode: ThemeMode) -> String {
    format!("{}: {}", crate::i18n::tr("Theme"), crate::i18n::tr(mode_label(mode)))
}

/// One suite button in its box, with the tooltip also as the control's name.
fn suite_button(ui: &mut Ui, id: &str, rect: Rect, icon: Icon, tooltip: &str) -> egui::Response {
    let mut inner = ui.new_child(UiBuilder::new().max_rect(rect).layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight)));
    let response = craft_icons::ui::button(&mut inner, icon, tooltip);
    announce(&response, tooltip);
    register(ui, format!("icon:{id}"), rect);
    response
}

/// The gear, the theme button and the search icon at the right of the bar, from the right edge: gear, theme, search.
/// `centre_y` is the bar's middle; returns the left edge of the last one (where the bar's own icons start).
pub fn buttons(app: &mut LocalPhotoApp, ui: &mut Ui, right: f32, centre_y: f32) -> f32 {
    let mut left = right - EDGE;
    let mut place = || {
        let rect = Rect::from_min_size(pos2(left - BOX, centre_y - BOX / 2.0), vec2(BOX, BOX));
        left -= BOX + GAP;
        rect
    };

    let settings = crate::i18n::tr("Settings");
    if suite_button(ui, "gear", place(), Icon::Settings, settings).clicked() {
        let _ = app.run("app.settings", json!({}));
    }

    let mode = theme_mode(app.ui.settings.appearance_mode);
    let tip = theme_text(mode);
    let rect = place();
    let response = suite_button(ui, "appearance", rect, mode.icon(), &tip);
    if response.clicked() {
        let _ = app.run("view.appearance", json!({"mode": mode_param(mode.next())}));
    }
    crate::widgets::context_menu(&response, |ui| appearance_menu(app, ui));

    let search = crate::i18n::tr("Search commands");
    if suite_button(ui, "palette", place(), Icon::Search, search).clicked() {
        app.palette.toggle();
    }
    left + GAP
}

/// The theme button's context menu: Appearance Settings and the favourite themes.
fn appearance_menu(app: &mut LocalPhotoApp, ui: &mut Ui) {
    let settings = ui.button(crate::i18n::tr("Appearance Settings…"));
    register(ui, "menu:appearanceSettings", settings.rect);
    if settings.clicked() {
        let _ = app.run("app.settings", json!({"tab": "interface"}));
        ui.close();
    }
    let favorites = app.ui.settings.favorite_themes.clone();
    for dark in [false, true] {
        let group: Vec<_> = crate::theme::ThemeKind::ALL.iter().filter(|k| k.is_dark() == dark && favorites.iter().any(|id| id == k.id())).collect();
        if !group.is_empty() {
            ui.separator();
        }
        for kind in group {
            let r = ui.selectable_label(app.ui.settings.theme(app.system_theme(ui.ctx())) == *kind, crate::i18n::tr(kind.label()));
            register(ui, format!("menu:themeFavorite-{}", kind.id()), r.rect);
            if r.clicked() {
                let _ = app.run("view.theme", json!({"theme": kind.id()}));
                ui.close();
            }
        }
    }
}

/// What the palette lists: every item of the menu bar, in menu order, with the menu (and submenu) as its group, the
/// shortcut the menu shows and the same enabled state. The id is the item's key; [`run`] resolves it back.
pub fn commands(app: &LocalPhotoApp, mac: bool) -> Vec<(Command, String, Value)> {
    fn walk(nodes: &[MenuNode], group: &str, mac: bool, out: &mut Vec<(Command, String, Value)>) {
        for node in nodes {
            match node {
                MenuNode::Item { id, params, label, shortcut, enabled, .. } => {
                    let key = MenuNode::key(id, params);
                    if out.iter().any(|(command, ..)| command.id == key) {
                        continue;
                    }
                    let mut command = Command::new(key, display_item_label(id, params, label), group).enabled(*enabled);
                    if let Some(shortcut) = shortcut {
                        command = command.shortcut(shortcut_text(shortcut, mac));
                    }
                    out.push((command, id.clone(), params.clone()));
                }
                MenuNode::Separator => {}
                MenuNode::Submenu { label, children } => walk(children, &format!("{group} › {}", crate::i18n::tr(label)), mac, out),
            }
        }
    }
    let mut out = Vec::new();
    for (title, items) in menu_bar(app) {
        walk(&items, crate::i18n::tr(&title), mac, &mut out);
    }
    out
}

/// The palette, when open: runs what the user picked, through the same code as the menu item.
pub fn palette(app: &mut LocalPhotoApp, ctx: &egui::Context) {
    if !app.palette.is_open() {
        return;
    }
    let listed = commands(app, ctx.os() == egui::os::OperatingSystem::Mac);
    let list: Vec<Command> = listed.iter().map(|(command, ..)| command.clone()).collect();
    let labels = Labels { placeholder: crate::i18n::tr("Search commands"), empty: crate::i18n::tr("No command matches.") };
    let Some(key) = app.palette.show(ctx, &labels, &list) else { return };
    if let Some((_, id, params)) = listed.into_iter().find(|(command, ..)| command.id == key)
        && let Err(error) = run_item(app, &id, params)
    {
        app.toast_error(ctx, error);
    }
}

/// A fresh palette (kept in the app).
pub fn new_palette() -> Palette {
    Palette::default()
}
