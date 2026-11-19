//! Onboarding flow: three phone screens of an invented trail app (welcome, pick what you like,
//! all set) that share a button component and a progress indicator, linked into a prototype.

use serde_json::json;
use trama_doc::NodeId;

use super::glyphs;
use super::kit::{ACCENT, BORDER, CARD, H, INK, Kit, MUTED, ON_BRAND, PAGE, PRIMARY, SOFT, SUNKEN, V};
use crate::Result;

const W: f64 = 390.0;
const SCREEN_H: f64 = 844.0;
const GAP: f64 = 80.0;

const INTERESTS: [(&str, bool); 8] = [
    ("Lakes", true),
    ("Summits", false),
    ("Forests", true),
    ("Coast", false),
    ("Family walks", false),
    ("Night skies", false),
    ("Bird watching", true),
    ("Long distance", false),
];

pub(super) fn build(k: &mut Kit) -> Result<()> {
    k.foundations()?;
    k.rename_page("Screens")?;
    let screens = k.page()?;
    let parts = k.add_page("Components")?;
    let button = button(k, parts)?;
    k.go(screens)?;
    let (welcome, next_a) = welcome(k, screens, button)?;
    let (interests, next_b) = interests(k, screens, button)?;
    let (ready, done) = ready(k, screens, button)?;
    for (from, to) in [(next_a, interests), (next_b, ready), (done, welcome)] {
        k.link(from, to)?;
    }
    k.flow(welcome, "Onboarding")
}

fn button(k: &mut Kit, page: NodeId) -> Result<NodeId> {
    let props = json!({"fill": PRIMARY, "cornerRadius": 18, "mainAlign": "center", "crossAlign": "center"});
    let c = k.stack_of("component", page, "Button", H, 8.0, json!([18, 24, 18, 24]), props)?;
    let label = k.text(c, "Button", "Label/M", ON_BRAND)?;
    k.text_prop(c, "Label", "Button", &[label])?;
    Ok(c)
}

fn screen(k: &mut Kit, page: NodeId, name: &str, index: usize) -> Result<NodeId> {
    let props = json!({"fill": PAGE, "stroke": BORDER, "strokeWeight": 1, "strokeAlign": "outside", "clip": true, "sizingH": "fixed", "sizingV": "fixed", "width": W, "height": SCREEN_H, "x": index as f64 * (W + GAP)});
    k.stack(page, name, V, 0.0, json!([56, 24, 32, 24]), props)
}

/// Three dots, the `current`th (from 0) long and filled.
fn progress(k: &mut Kit, parent: NodeId, current: usize) -> Result<()> {
    let dots = k.stack(parent, "Progress", H, 6.0, json!(0), json!({"crossAlign": "center"}))?;
    for i in 0..3 {
        let (width, fill) = if i == current { (24.0, PRIMARY) } else { (8.0, SUNKEN) };
        k.rect(dots, &format!("Step {}", i + 1), [0.0, 0.0, width, 8.0], json!({"fill": fill, "cornerRadius": 4}))?;
    }
    Ok(())
}

fn welcome(k: &mut Kit, page: NodeId, button: NodeId) -> Result<(NodeId, NodeId)> {
    let s = screen(k, page, "Welcome", 0)?;
    k.set(s, json!({"padding": [0, 0, 32, 0]}))?;
    k.scene(s, "Picture", [W, 460.0], json!({"sizingH": "fill"}))?;
    let body = k.stack(s, "Content", V, 0.0, json!([36, 24, 0, 24]), json!({"sizingH": "fill", "sizingV": "fill", "mainAlign": "spaceBetween"}))?;
    let words = k.stack(body, "Words", V, 12.0, json!(0), json!({"sizingH": "fill"}))?;
    k.text(words, "WELCOME", "Label/S", PRIMARY)?;
    k.para(words, "Walk more, plan less", "Heading/L", INK)?;
    k.para(words, "Tell us what you enjoy and we will line up routes that fit your weekend.", "Body/M", MUTED)?;
    let actions = k.stack(body, "Actions", V, 20.0, json!(0), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    progress(k, actions, 0)?;
    let next = k.instance(button, actions, json!({"Label": "Continue"}), json!({"sizingH": "fill"}))?;
    Ok((s, next))
}

fn interests(k: &mut Kit, page: NodeId, button: NodeId) -> Result<(NodeId, NodeId)> {
    let s = screen(k, page, "Interests", 1)?;
    let body = k.stack(s, "Content", V, 0.0, json!(0), json!({"sizingH": "fill", "sizingV": "fill", "mainAlign": "spaceBetween"}))?;
    let words = k.stack(body, "Words", V, 20.0, json!(0), json!({"sizingH": "fill"}))?;
    k.text(words, "STEP 2 OF 3", "Label/S", PRIMARY)?;
    k.para(words, "What do you like to walk?", "Heading/L", INK)?;
    k.para(words, "Pick a few. You can change them later in your profile.", "Body/M", MUTED)?;
    let choices = k.stack(words, "Interests", V, 10.0, json!([12, 0, 0, 0]), json!({"sizingH": "fill"}))?;
    for pair in INTERESTS.chunks(2) {
        let row = k.stack(choices, "Row", H, 10.0, json!(0), json!({"sizingH": "fill"}))?;
        for (label, on) in pair {
            let (fill, color) = if *on { (PRIMARY, ON_BRAND) } else { (CARD, INK) };
            let chip = k.chip(row, label, ["Label/M", color], fill, [14.0, 18.0], 16.0)?;
            k.set(chip, json!({"sizingH": "fill", "stroke": if *on { PRIMARY } else { BORDER }, "strokeWeight": 1}))?;
            if *on {
                k.icon(chip, &glyphs::CHECK, 18.0, ON_BRAND)?;
            }
        }
    }
    let actions = k.stack(body, "Actions", V, 20.0, json!(0), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    progress(k, actions, 1)?;
    let next = k.instance(button, actions, json!({"Label": "Show me routes"}), json!({"sizingH": "fill"}))?;
    Ok((s, next))
}

fn ready(k: &mut Kit, page: NodeId, button: NodeId) -> Result<(NodeId, NodeId)> {
    let s = screen(k, page, "All set", 2)?;
    let body = k.stack(s, "Content", V, 0.0, json!(0), json!({"sizingH": "fill", "sizingV": "fill", "mainAlign": "spaceBetween"}))?;
    let centre = k.stack(body, "Message", V, 20.0, json!([96, 0, 0, 0]), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    let badge = k.tile(centre, 112.0, 56.0)?;
    k.set(badge, json!({"fill": SOFT}))?;
    k.icon(badge, &glyphs::CHECK, 48.0, PRIMARY)?;
    k.text_with(centre, "You are all set", "Heading/L", INK, json!({"textAlign": "center"}))?;
    k.text_with(
        centre,
        "Three lakes, two forests and a bird hide are waiting within an hour of you.",
        "Body/M",
        MUTED,
        json!({"sizingH": "fill", "textAutoResize": "height", "textAlign": "center"}),
    )?;
    let tip = k.stack(
        centre,
        "Tip",
        H,
        10.0,
        json!(14),
        json!({"fill": CARD, "cornerRadius": 16, "stroke": BORDER, "strokeWeight": 1, "crossAlign": "center", "sizingH": "fill"}),
    )?;
    k.icon(tip, &glyphs::BELL, 20.0, ACCENT)?;
    k.text_with(tip, "We will tell you when the weather is right.", "Body/S", MUTED, json!({"sizingH": "fill"}))?;
    let actions = k.stack(body, "Actions", V, 20.0, json!(0), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    progress(k, actions, 2)?;
    let done = k.instance(button, actions, json!({"Label": "Open my routes"}), json!({"sizingH": "fill"}))?;
    Ok((s, done))
}
