//! Software updates: Help ▸ Check for Updates, the check at start, and their windows.
//!
//! The flow and the windows are `nodalia-update`'s, shared with the other Nodalia apps: it replaces
//! an AppImage or `Trama.app`, installs the MSI on Windows, and points other installs at the
//! release page. This file is what is Trama's own: when an update may close the app.

use crate::FigApp;

#[cfg(target_arch = "wasm32")]
use crate::i18n::tr;
#[cfg(not(target_arch = "wasm32"))]
use nodalia_update::egui::{Action, Updater};

pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    #[cfg(not(target_arch = "wasm32"))]
    repaint: Option<egui::Context>,
    /// The result dialog of a manual check is open.
    #[cfg(target_arch = "wasm32")]
    pub(crate) manual_window_open: bool,
    /// The web build has no updater; its only result is that checks are unsupported.
    #[cfg(target_arch = "wasm32")]
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(crate::UPDATE_APP).with_accent(crate::UPDATE_ACCENT),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            repaint: None,
            #[cfg(target_arch = "wasm32")]
            manual_window_open: false,
            #[cfg(target_arch = "wasm32")]
            status_message: None,
        }
    }
}

impl FigApp {
    /// Whether the app asks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut FigApp) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.check_requested = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some(tr("Update check is not supported in the web browser.").to_string());
    }
}

pub(crate) fn tick(app: &mut FigApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        let action = rt.updater.poll();
        act(ctx, action);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

#[cfg(not(target_arch = "wasm32"))]
fn act(ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
        // The new version is starting, or a helper waits for this process to end. The install only
        // ran once nothing was unsaved, so the root window's guarded close goes through; exit anyway
        // if it has not closed soon.
        Action::Exit => {
            ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut FigApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // An install waits until no document holds unsaved changes.
        let clean = !app.session.docs.iter().any(|d| d.is_dirty());
        let language = nodalia_update::Language::from_code(crate::i18n::current().map_or("en", |l| l.code));
        let action = app.updates_rt.updater.show(ctx, language, clean);
        act(ctx, action);
    }

    #[cfg(target_arch = "wasm32")]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        let mut close = false;
        let t = crate::theme::Tokens::get(ctx);
        crate::dialogs::ndl_window(tr("Check for Updates"), ctx).id(egui::Id::unique("manual-update-check")).show(ctx, |ui| {
            if crate::dialogs::header(ui, &t, tr("Check for Updates")) {
                open = false;
            }
            if let Some(msg) = &app.updates_rt.status_message {
                ui.label(msg);
            }
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(tr("Close")).clicked() {
                    close = true;
                }
            });
        });
        if !open || close {
            app.updates_rt.manual_window_open = false;
            app.updates_rt.status_message = None;
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use nodalia_update::egui::Status;

    #[test]
    fn the_releases_are_tramas_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/trama");
        assert_eq!(crate::UPDATE_APP.bundle, "Trama.app");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    /// The names Trama's release workflows give its packages, and the systems whose updater must
    /// find each one: a rename in `packaging/` the updater does not know would strand those users.
    #[test]
    fn every_published_package_is_found_by_the_updater_it_is_for() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../packaging");
        let read = |path: &str| std::fs::read_to_string(format!("{root}/{path}")).unwrap();
        let macos = read("macos/package.sh");
        assert!(macos.contains(r#"APP="$WORK/Trama.app""#), "the bundle the updater swaps");
        assert!(macos.contains(r#"DMG="$DIST/trama-$VERSION-macos-$ARCH.dmg""#));
        for (package, systems) in [
            ("trama-1.0.0-macos-universal.dmg", &[("macos", "aarch64"), ("macos", "x86_64")][..]),
            ("trama-1.0.0-linux-x86_64.AppImage", &[("linux", "x86_64")]),
            ("trama-1.0.0-windows-x64.msi", &[("windows", "x86_64")]),
        ] {
            let found = nodalia_update::systems_for_package(package);
            for system in systems {
                assert!(found.contains(system), "{package} is not offered to {system:?}");
            }
        }
    }

    #[test]
    fn a_check_at_start_is_silent_until_it_finds_something() {
        let mut rt = Runtime::default();
        assert!(!rt.updater.is_open());
        rt.updater.set_status(Status::Checking { user: false });
        assert!(!rt.updater.is_open());
        rt.updater.set_status(Status::Checking { user: true });
        assert!(rt.updater.is_open());
    }
}
