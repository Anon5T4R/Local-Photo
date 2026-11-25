//! Community and project links (Help menu, About dialog).

/// The app's id, as the repository and the release assets spell it.
pub const APP: &str = "localphoto";
/// This app's page: the fork's repository (there is no separate product website).
pub const APP_PAGE: &str = "https://github.com/Styx-Studios/Local-Photo";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/Styx-Studios/Local-Photo";

/// (UI command id, menu label, URL) for each link, in Help-menu order.
/// The user documentation (docs/ in the repository).
pub const HELP: &str = "https://github.com/Styx-Studios/Local-Photo/tree/main/docs";

pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "Local Photo Help", HELP),
    ("app.website", "Local Photo Website", APP_PAGE),
    ("app.github", "Local Photo on GitHub", GITHUB),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

/// Where feedback and bug reports go.
pub const FEEDBACK: &str = "https://github.com/Styx-Studios/Local-Photo/issues/new";

/// The URL behind a link command id.
pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LocalPhotoApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_https_and_name_this_app() {
        for (id, label, url) in LINKS {
            assert!(url.starts_with("https://"), "{id}");
            assert!(!label.is_empty());
            assert_eq!(url_of(id), Some(*url));
        }
        assert!(APP_PAGE.ends_with("/Local-Photo"));
        assert_eq!(GITHUB, APP_PAGE);
    }
}
