//! Newsletter: one 640-wide email of an invented monthly letter (masthead, picture, a lead story,
//! three reads, a quote and a footer) on shared styles and auto layout.

use serde_json::json;
use trama_doc::NodeId;

use super::glyphs;
use super::kit::{ACCENT, BORDER, BOTTOM_EDGE, CARD, DEEP, H, INK, Kit, MUTED, ON_BRAND, PAGE, PRIMARY, SOFT, V};
use crate::Result;

const W: f64 = 640.0;

/// The reads: (title, who and how long).
const READS: [(&str, &str); 3] = [
    ("Seven small climbs with big views", "Ines Okafor · 6 min"),
    ("How to read a contour map in ten minutes", "Tomas Reyes · 9 min"),
    ("A thermos, two apples and a plan", "Mara Lindqvist · 4 min"),
];

pub(super) fn build(k: &mut Kit) -> Result<()> {
    k.foundations()?;
    k.rename_page("Email")?;
    let page = k.page()?;
    let props = json!({"fill": CARD, "stroke": BORDER, "strokeWeight": 1, "strokeAlign": "outside", "clip": true, "sizingH": "fixed", "width": W, "x": 0, "y": 0});
    let root = k.stack(page, "Newsletter", V, 0.0, json!(0), props)?;
    masthead(k, root)?;
    k.scene(root, "Picture", [W, 260.0], json!({"sizingH": "fill"}))?;
    lead(k, root)?;
    reads(k, root)?;
    quote(k, root)?;
    footer(k, root)
}

fn masthead(k: &mut Kit, root: NodeId) -> Result<()> {
    let bar =
        k.stack(root, "Masthead", H, 12.0, json!([24, 40, 24, 40]), json!({"sizingH": "fill", "mainAlign": "spaceBetween", "crossAlign": "center"}))?;
    let logo = k.stack(bar, "Logo", H, 10.0, json!(0), json!({"crossAlign": "center"}))?;
    k.icon(logo, &glyphs::MOUNTAIN, 28.0, PRIMARY)?;
    k.text(logo, "The Ridgeline Letter", "Heading/S", INK)?;
    k.text(bar, "No. 27 · June", "Label/S", MUTED).map(drop)
}

fn lead(k: &mut Kit, root: NodeId) -> Result<()> {
    let story = k.stack(root, "Lead story", V, 14.0, json!([36, 40, 36, 40]), json!({"sizingH": "fill"}))?;
    k.text(story, "THIS MONTH", "Label/S", PRIMARY)?;
    k.para(story, "The longest day is the best day to start early", "Heading/L", INK)?;
    k.para(story, "On the solstice the light lasts sixteen hours. We asked six guides where they would spend them, and every one of them started before sunrise.", "Body/M", MUTED)?;
    let button = k.chip(story, "Read the story", ["Label/M", ON_BRAND], PRIMARY, [12.0, 22.0], 14.0)?;
    k.set(button, json!({"sizingH": "hug"}))
}

fn reads(k: &mut Kit, root: NodeId) -> Result<()> {
    let section = k.stack(root, "Reads", V, 0.0, json!([28, 40, 12, 40]), json!({"fill": PAGE, "sizingH": "fill"}))?;
    k.text(section, "Three good reads", "Heading/M", INK)?;
    for (i, (title, meta)) in READS.into_iter().enumerate() {
        let props = json!({"sizingH": "fill", "crossAlign": "center", "stroke": BORDER, "strokeSides": BOTTOM_EDGE, "strokeWeight": if i + 1 < READS.len() { 1 } else { 0 }});
        let row = k.stack(section, title, H, 16.0, json!([18, 0, 18, 0]), props)?;
        let number = k.tile(row, 44.0, 14.0)?;
        k.text(number, &format!("{}", i + 1), "Heading/S", PRIMARY)?;
        let words = k.stack(row, "Words", V, 2.0, json!(0), json!({"sizingH": "fill"}))?;
        k.para(words, title, "Heading/S", INK)?;
        k.text(words, meta, "Body/S", MUTED)?;
        k.icon(row, &glyphs::ARROW_RIGHT, 20.0, MUTED)?;
    }
    Ok(())
}

fn quote(k: &mut Kit, root: NodeId) -> Result<()> {
    let block = k.stack(root, "Quote", V, 12.0, json!([36, 40, 36, 40]), json!({"fill": SOFT, "sizingH": "fill"}))?;
    k.para(block, "“The trail does not care how fast you are. It only asks that you turn up.”", "Heading/M", INK)?;
    k.text(block, "Ines Okafor, mountain guide", "Label/M", PRIMARY).map(drop)
}

fn footer(k: &mut Kit, root: NodeId) -> Result<()> {
    let bar = k.stack(root, "Footer", V, 8.0, json!([28, 40, 32, 40]), json!({"fill": DEEP, "sizingH": "fill", "crossAlign": "center"}))?;
    k.icon(bar, &glyphs::MAIL, 24.0, ACCENT)?;
    k.text_with(bar, "You get this letter because you joined Ridgeline Trails.", "Body/S", "#CFE6DE", json!({"textAlign": "center"}))?;
    k.text(bar, "Unsubscribe · Update preferences · View in a browser", "Label/S", ON_BRAND).map(drop)
}
