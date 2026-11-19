//! The top bar: sidebar toggle, back/forward, search, filter, and the right-hand icons.

use egui::{Align2, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use serde_json::json;

use crate::LocalPhotoApp;
use crate::icons::{Icon, paint};
use crate::theme::Tokens;
use crate::widgets::{icon_button, register};

/// The bar doubles as the window's title bar: dragging its empty space moves the window and a
/// double-click acts as the system setting says. Made before the bar's widgets so they win the click.
fn window_handle(ui: &mut egui::Ui, bar: Rect) {
    register(ui, "region:titlebar", bar);
    let resp = craft_window::title_bar::empty_area(ui, bar);
    crate::widgets::announce(&resp, crate::i18n::tr("Title bar"));
}

/// Draw the top bar: the titlebar drag area, the in-window menus when there is no native menu
/// bar, sidebar and back/forward buttons, the centred search field, the filter button with its
/// active count, and the right-side action icons.
pub fn show(app: &mut LocalPhotoApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    // room for the traffic lights; in full screen they are gone, and the sidebar button sits as far
    // from the left edge as the last bar icon from the right (12 margin + 4)
    let left = match (app.integrated_titlebar, app.window_is_fullscreen) {
        (true, false) => 78,
        (true, true) => 16,
        (false, _) => 10,
    };
    egui::Panel::top("top_bar")
        .exact_size(t.top_bar_h)
        .frame(egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin { left, right: 12, top: 0, bottom: 0 }))
        .show(ui, |ui| {
            let full = ui.max_rect();
            // The whole bar, its margins included. The search field is centred on this and not on
            // `full`, which is only what the margins leave: on macOS the left margin holds the
            // traffic lights' room, and full screen gives that room back. Centring on `full` moved
            // the field, and changed its width, every time that room changed (issue #2365).
            let bar = Rect::from_min_max(pos2(full.left() - left as f32, full.top()), pos2(full.right() + 12.0, full.bottom()));
            // The field stops short of the filter icon and the eight icons on the right; centred,
            // it is twice the room between the window's centre and them.
            let mut sw = 640.0f32.min(2.0 * (full.right() - (310.0 + crate::shell::REACH - 40.0) - bar.center().x)).max(190.0);
            if !app.native_menu {
                // leave room for the in-window menus left of the (centred) search field
                let menus_right = full.left() + 140.0 + crate::menubar::bar_width(ui) + 24.0;
                sw = sw.min(2.0 * (bar.center().x - menus_right)).max(190.0);
            }
            let search_left = bar.center().x - sw / 2.0;
            let sr = Rect::from_center_size(pos2(bar.center().x, full.center().y), vec2(sw, 28.0));
            if app.integrated_titlebar {
                window_handle(ui, bar);
            }
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if icon_button(ui, "sidebar", Icon::Sidebar, vec2(30.0, 30.0), app.ui.left_panel, true, crate::i18n::tr("Show/hide My Photos panel"))
                    .clicked()
                {
                    let _ = app.run("view.leftPanel", json!({}));
                }
                ui.add_space(8.0);
                if icon_button(ui, "back", Icon::Back, vec2(30.0, 30.0), false, true, crate::i18n::tr("Back")).clicked() {
                    let _ = app.run("view.back", json!({}));
                }
                icon_button(ui, "forward", Icon::Forward, vec2(30.0, 30.0), false, false, crate::i18n::tr("Forward"));
                if !app.native_menu {
                    // no native menu bar (web, Windows, Linux): menus in the top bar
                    ui.add_space(10.0);
                    let room = search_left - ui.cursor().left() - 16.0;
                    crate::menubar::show_in_window(app, ui, room);
                }
            });
            // search field (centred on the window)
            let id = crate::text_field::id("field:search");
            if std::mem::take(&mut app.ui.focus_search) {
                ui.memory_mut(|m| m.request_focus(id));
            }
            let focused = ui.memory(|m| m.has_focus(id));
            ui.painter().rect(
                sr,
                4.0,
                if focused { t.canvas } else { t.field },
                Stroke::new(1.0, if focused { t.accent } else { t.field_border }),
                StrokeKind::Inside,
            );
            let mut child =
                ui.new_child(egui::UiBuilder::new().max_rect(sr.shrink2(vec2(10.0, 4.0))).layout(egui::Layout::left_to_right(egui::Align::Center)));
            let empty = app.ui.search.is_empty();
            if empty && !focused {
                let g = child.painter().layout_no_wrap(crate::i18n::tr("Search Photos").into(), t.font(13.5), t.text_dim);
                let w = g.size().x + 24.0;
                let x0 = sr.center().x - w / 2.0;
                paint(child.painter(), Rect::from_min_size(pos2(x0, sr.center().y - 8.0), vec2(16.0, 16.0)), Icon::Search, t.text_dim);
                child.painter().galley(pos2(x0 + 24.0, sr.center().y - g.size().y / 2.0), g, t.text_dim);
            }
            let resp = crate::text_field::TextField::singleline("field:search", &mut app.ui.search)
                .label(crate::i18n::tr("Search Photos"))
                .frame(false)
                .width(sr.width() - 20.0)
                .font(t.font(13.5))
                .text_color(t.text)
                .show(&mut child)
                .response;
            if resp.changed() {
                let q = app.ui.search.clone();
                let _ = app.run("library.filter", json!({"text": q}));
            }
            // filter icon right of the search field
            let fr = Rect::from_center_size(pos2(sr.right() + 22.0, sr.center().y), vec2(28.0, 28.0));
            let fresp = ui.interact(fr, egui::Id::unique("filter-btn"), Sense::click());
            crate::widgets::announce(&fresp, crate::i18n::tr("Filters"));
            register(ui, "icon:filter", fr);
            let filtering = app.session.filter != Default::default() || app.ui.filter_bar;
            paint(
                ui.painter(),
                fr.shrink(6.0),
                Icon::Filter,
                if filtering {
                    t.accent
                } else if fresp.hovered() {
                    t.text
                } else {
                    t.icon
                },
            );
            // badge: how many filters are on, even with the filter bar closed
            let active = localphoto_engine::filter_chips(&app.session.filter, &app.session.catalog).len();
            if active > 0 {
                let c = fr.right_top() + vec2(-3.0, 8.0);
                ui.painter().circle_filled(c, 7.0, t.accent);
                ui.painter().text(c, Align2::CENTER_CENTER, active.to_string(), t.semibold(9.5), t.canvas);
            }
            let fresp = fresp.on_hover_text(if active > 0 {
                crate::i18n::tr_format!("Filter bar — {active} active filter{}", if active == 1 { "" } else { "s" }, active = active)
            } else {
                crate::i18n::tr("Filter bar").into()
            });
            if fresp.clicked() {
                let _ = app.run("view.filterBar", json!({}));
            }
            // the suite's buttons at the right edge: Settings, theme, search commands
            let suite_left = crate::shell::buttons(app, ui, full.right(), full.center().y);
            // right icons
            let mut x = suite_left - 6.0 - 14.0;
            // Saving is failing: a warning shows until a save succeeds. When everything is written there is
            // nothing to say, so there is no icon (the library is local; a cloud badge suggested a sync it never had).
            let unsaved = app.session.unsaved().map(|(n, e)| {
                crate::i18n::tr_format!(
                    "{n} change{} saved in memory but not written to disk: {e}\nLocal Photo retries automatically; quitting now would lose {}.",
                    if n == 1 { "" } else { "s" },
                    if n == 1 { "it" } else { "them" },
                    e = e,
                    n = n
                )
            });
            // Import Photos… beside Export, always one click away, with the key it is bound to now
            let mac = ui.ctx().os() == egui::os::OperatingSystem::Mac;
            let import_tip = match crate::shortcuts::shortcut_of(&app.ui.settings.keymap, "file.addPhotos") {
                Some(sc) => format!("{} ({})", crate::i18n::tr("Import Photos…"), crate::menubar::shortcut_text(sc, mac)),
                None => crate::i18n::tr("Import Photos…").to_string(),
            };
            let mut icons: Vec<(&str, Icon, &str, &str)> = Vec::with_capacity(5);
            if let Some(tip) = unsaved.as_deref() {
                icons.push(("unsaved", Icon::Warning, tip, ""));
            }
            icons.extend([
                ("help", Icon::Help, "Keyboard shortcuts", "app.shortcuts"),
                ("share", Icon::Share, "Export", "dialog.export"),
                ("import", Icon::Plus, import_tip.as_str(), "file.addPhotos"),
                ("bell", Icon::Bell, "Activity", "panel.activity"),
            ]);
            for (id, icon, tip, cmd) in icons {
                let r = Rect::from_center_size(pos2(x, full.center().y), vec2(28.0, 28.0));
                let resp = ui.interact(r, egui::Id::unique(("top", id)), Sense::click()).on_hover_text(crate::i18n::tr(tip));
                crate::widgets::announce(&resp, crate::i18n::tr(tip));
                register(ui, format!("icon:{id}"), r);
                let warn = id == "unsaved";
                let colour = if warn {
                    t.caution
                } else if resp.hovered() {
                    t.text
                } else {
                    t.icon
                };
                paint(ui.painter(), r.shrink(5.0), icon, colour);
                if warn {
                    let c = r.right_top() + vec2(-5.0, 6.0);
                    ui.painter().circle_filled(c, 6.0, t.reject);
                    ui.painter().text(c, Align2::CENTER_CENTER, "!", t.semibold(9.5), t.canvas);
                    register(ui, "indicator:unsaved", r);
                }
                // a refusal says why (Import while a Lightroom catalog is coming in)
                if resp.clicked()
                    && !cmd.is_empty()
                    && let Err(e) = app.run(cmd, json!({}))
                {
                    app.toast_error(ui.ctx(), e);
                }
                x -= 40.0;
            }
            let _ = Align2::CENTER_CENTER;
        });
}
