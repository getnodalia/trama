//! Every screen draws its text in the interface language. The dialog or panel behind every menu
//! command is drawn in Spanish; a text that is still the English key of a Spanish catalog row was
//! drawn without going through the catalog.
use std::collections::{BTreeMap, BTreeSet};

use egui::epaint::Shape;
use serde_json::json;
use trama_engine::Session;
use trama_ui_egui::NoServices;
use trama_ui_egui::{FigApp, i18n, menus};

/// Names of what the app ships or the walk makes, drawn as stored: a file, a page, a layer, a
/// style or a variable is data the person can rename, so it is not interface text.
const DATA: &[&str] = &[
    // A key name, drawn as the key cap it stands for.
    "Enter",
];

/// Texts whose draw site is still to be found. Each is a bug, never a place to add to: remove one
/// when its site goes through the catalog.
const KNOWN: &[&str] = &[];

struct Screen {
    ctx: egui::Context,
    app: FigApp,
}

impl Screen {
    fn new() -> Self {
        let mut s = Session::new();
        s.execute("node.create", &json!({"type": "frame", "x": 40, "y": 40, "width": 300, "height": 200})).unwrap();
        let mut app = FigApp::new(s, Box::new(NoServices));
        app.session.prefs.language = "es".into();
        let mut screen = Screen { ctx: egui::Context::default(), app };
        screen.frames(4, Vec::new());
        screen
    }

    /// Runs `n` frames; `events` go in the first one. Returns the text the last frame painted.
    fn frames(&mut self, n: usize, events: Vec<egui::Event>) -> BTreeMap<String, String> {
        let mut shown = BTreeMap::new();
        for i in 0..n {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 900.0))),
                events: if i == 0 { events.clone() } else { Vec::new() },
                ..Default::default()
            };
            let mut out = self.ctx.run_ui(input, |ui| self.app.ui(ui));
            out.textures_delta.clear();
            shown.clear();
            for clipped in &out.shapes {
                collect_shape(&clipped.shape, &mut shown);
            }
        }
        shown
    }

    fn escape(&mut self) {
        let key = |pressed| egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: Some(egui::Key::Escape),
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        for _ in 0..3 {
            self.frames(2, vec![key(true), key(false)]);
        }
    }
}

fn collect_shape(shape: &Shape, out: &mut BTreeMap<String, String>) {
    match shape {
        Shape::Text(t) => {
            for line in t.galley.text().split('\n') {
                let line = line.trim();
                if !line.is_empty() {
                    out.entry(line.to_owned()).or_insert_with(|| "painted".to_owned());
                }
            }
        }
        Shape::Vec(v) => v.iter().for_each(|s| collect_shape(s, out)),
        _ => {}
    }
}

/// Texts of the frame that are English keys of a Spanish row with another reading.
fn untranslated(frame: &BTreeMap<String, String>) -> Vec<String> {
    frame
        .iter()
        .filter(|(t, _)| !DATA.contains(&t.as_str()) && !KNOWN.contains(&t.as_str()))
        .filter(|(t, _)| i18n::try_tr(t).is_some_and(|es| es != t.as_str()))
        .map(|(t, kind)| format!("{t:?} ({kind})"))
        .collect()
}

/// Commands that leave the app or close the file the walk draws on.
fn skipped(id: &str) -> bool {
    id.contains("quit") || id.contains("exit") || id.starts_with("file.close") || id == "window.new" || id.is_empty()
}

fn walk() -> Vec<String> {
    let mut failures: Vec<String> = Vec::new();
    let mut seen = BTreeSet::new();
    let mut note = |what: &str, frame: &BTreeMap<String, String>, failures: &mut Vec<String>| {
        for line in untranslated(frame) {
            if seen.insert(line.clone()) {
                failures.push(format!("{what}: {line}"));
            }
        }
    };
    let mut screen = Screen::new();
    let main = screen.frames(2, Vec::new());
    assert_eq!(i18n::current().map(|l| l.code), Some("es"), "the frame sets the language");
    assert!(main.len() > 10, "the main window paints its text: {main:?}");
    note("main window", &main, &mut failures);
    for id in trama_engine::catalog::items().map(|m| m.id).collect::<BTreeSet<_>>() {
        if skipped(id) {
            continue;
        }
        menus::invoke(&mut screen.app, id);
        let frame = screen.frames(3, Vec::new());
        note(&format!("command {id}"), &frame, &mut failures);
        screen.escape();
    }
    failures
}

/// A screen that draws a text the catalog translates, without going through the catalog, shows
/// English to everyone who runs Trama in Spanish.
#[test]
fn spanish_interface_has_no_untranslated_text() {
    let failures = walk();
    assert!(failures.is_empty(), "texts drawn in English although Spanish has them:\n{}", failures.join("\n"));
}
