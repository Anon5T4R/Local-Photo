//! Software updates, background check at startup, and startup prompt modal.

use serde_json::Value;

use crate::LocalPhotoApp;

use craft_update::updater::{Action, Status, Updater};

pub(crate) struct Runtime {
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    startup_check: bool,
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    repaint: Option<egui::Context>,
    pub(crate) manual_window_open: bool,
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            updater: Updater::new(&craft_update::suite::LOCALPHOTO, env!("CARGO_PKG_VERSION")),
            startup_check: false,
            check_requested: false,
            repaint: None,
            manual_window_open: false,
            status_message: None,
        }
    }
}

impl LocalPhotoApp {
    /// Whether the app asks for a newer release when it starts (off until the desktop app turns it on).
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }

    /// Test only: a newer release is on offer, so the update prompt is up.
    #[cfg(test)]
    pub(crate) fn offer_update_for_test(&mut self, name: &str) {
        self.updates_rt.updater.offer(craft_update::Release {
            tag: "v99.0.0".into(),
            version: craft_update::parse_version("99.0.0"),
            name: name.into(),
            html_url: "https://github.com/Styx-Studios/Local-Photo/releases/tag/v99.0.0".into(),
            ..Default::default()
        });
    }
}

/// The update prompt owns the keyboard while it is open: shortcuts must not reach the library behind it.
pub(crate) fn blocks_input(app: &LocalPhotoApp) -> bool {
    app.updates_rt.updater.prompt_open()
}

pub(crate) fn check(app: &mut LocalPhotoApp) -> Result<Value, String> {
    {
        app.updates_rt.check_requested = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
        Ok(Value::Null)
    }
}

pub(crate) fn tick(app: &mut LocalPhotoApp, ctx: &egui::Context) {
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        rt.updater.tick(ctx);
    }
}

pub(crate) fn show(app: &mut LocalPhotoApp, ctx: &egui::Context) {
    {
        // The restart helper is scheduled before the app closes, so no update starts over unsaved work.
        let clean = app.session.unsaved().is_none();
        match app.updates_rt.updater.show_if(ctx, clean) {
            Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
            Action::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Action::None => {}
        }
        // The window answers Help ▸ Check for Updates, so it follows the updater's status.
        let rt = &mut app.updates_rt;
        (rt.manual_window_open, rt.status_message) = match rt.updater.status() {
            Status::Idle => (false, None),
            Status::Checking => (true, Some(crate::i18n::tr("Checking for updates…").to_string())),
            Status::UpToDate => (
                true,
                Some(
                    crate::i18n::tr("Local Photo {version} is currently the newest version available.")
                        .replace("{version}", env!("CARGO_PKG_VERSION")),
                ),
            ),
            Status::Failed(e) => (true, Some(crate::i18n::tr("Could not check for updates:\n{error}").replace("{error}", e))),
        };
    }

    if app.updates_rt.manual_window_open {
        let mut open = true;
        egui::Window::new(crate::i18n::tr("Local Photo Updates"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                if ui.button(crate::i18n::tr("OK")).clicked() {
                    app.updates_rt.manual_window_open = false;
                }
            });
        if !open {
            app.updates_rt.manual_window_open = false;
        }
    }

    // Closing the window dismisses the outcome it showed.
    if !app.updates_rt.manual_window_open {
        app.updates_rt.updater.dismiss_status();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use localphoto_engine::Session;

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        assert!(!rt.manual_window_open);
        {
            assert!(!rt.startup_check);
            assert!(!rt.updater.prompt_open());
            assert_eq!(rt.updater.status(), &Status::Idle);
        }
    }

    #[test]
    fn startup_prompt_shows_and_closes() {
        let mut app = LocalPhotoApp::new(Session::new(), Default::default());
        let ctx = egui::Context::default();
        let release = craft_update::Release {
            tag: "v99.0.0".into(),
            version: craft_update::parse_version("99.0.0"),
            name: "Local Photo 99.0.0".into(),
            html_url: "https://github.com/Styx-Studios/Local-Photo/releases/tag/v99.0.0".into(),
            ..Default::default()
        };
        app.updates_rt.updater.offer(release);

        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            show(&mut app, ui.ctx());
        });
        output.textures_delta.clear();
        assert!(app.updates_rt.updater.prompt_open());
    }
}
