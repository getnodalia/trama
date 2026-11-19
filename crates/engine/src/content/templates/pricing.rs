//! Pricing page: a 1440-wide page of an invented plan list (a header, a heading, three plan
//! cards with the middle one raised, questions and a footer) on shared styles and auto layout.

use serde_json::json;
use trama_doc::NodeId;

use super::glyphs;
use super::kit::{ACCENT, BORDER, BOTTOM_EDGE, CARD, DEEP, H, INK, Kit, MUTED, ON_BRAND, PAGE, PRIMARY, SOFT, V, shadow};
use crate::Result;

const W: f64 = 1440.0;

/// A plan: (name, price, what it is for, features, the one to start with).
type Plan = (&'static str, &'static str, &'static str, &'static [&'static str], bool);

const PLANS: [Plan; 3] = [
    (
        "Free",
        "$0",
        "For a first season on your own",
        &["3 saved routes", "Offline maps for one region", "Basic trip notes", "Community support"],
        false,
    ),
    (
        "Club",
        "$9",
        "For a walking group that meets every week",
        &["Unlimited routes", "Offline maps everywhere", "Shared trip notes", "Group calendar", "Priority support"],
        true,
    ),
    (
        "Guide",
        "$24",
        "For people who lead walks for a living",
        &["Everything in Club", "Bookings and payments", "Printed route cards", "Branded pages"],
        false,
    ),
];

const QUESTIONS: [(&str, &str); 3] = [
    ("Can I change plan later?", "Yes. Moving up takes effect straight away; moving down starts at the next renewal."),
    ("Is there a discount for the year?", "Paying yearly takes two months off every plan, and you can switch back at any time."),
    ("What happens to my routes if I leave?", "They stay yours: export every route as a file before you go, or keep them on the free plan."),
];

pub(super) fn build(k: &mut Kit) -> Result<()> {
    k.foundations()?;
    k.rename_page("Pricing")?;
    let page = k.page()?;
    let props = json!({"fill": PAGE, "stroke": BORDER, "strokeWeight": 1, "strokeAlign": "outside", "clip": true, "sizingH": "fixed", "width": W, "x": 0, "y": 0});
    let root = k.stack(page, "Pricing page", V, 0.0, json!(0), props)?;
    header(k, root)?;
    heading(k, root)?;
    plans(k, root)?;
    questions(k, root)?;
    footer(k, root)
}

fn header(k: &mut Kit, root: NodeId) -> Result<()> {
    let bar =
        k.stack(root, "Header", H, 24.0, json!([24, 96, 24, 96]), json!({"sizingH": "fill", "mainAlign": "spaceBetween", "crossAlign": "center"}))?;
    let logo = k.stack(bar, "Logo", H, 10.0, json!(0), json!({"crossAlign": "center"}))?;
    k.icon(logo, &glyphs::MOUNTAIN, 32.0, PRIMARY)?;
    k.text(logo, "Ridgeline Trails", "Heading/S", INK)?;
    let links = k.stack(bar, "Links", H, 36.0, json!(0), json!({"crossAlign": "center"}))?;
    for label in ["Routes", "Pricing", "Guides", "Help"] {
        k.text(links, label, "Label/M", if label == "Pricing" { PRIMARY } else { MUTED })?;
    }
    k.chip(bar, "Sign in", ["Label/M", ON_BRAND], PRIMARY, [12.0, 22.0], 999.0).map(drop)
}

fn heading(k: &mut Kit, root: NodeId) -> Result<()> {
    let words = k.stack(root, "Heading", V, 16.0, json!([72, 96, 48, 96]), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    k.text(words, "PRICING", "Label/S", PRIMARY)?;
    k.text(words, "Simple plans for every pace", "Display", INK)?;
    k.text_with(
        words,
        "Start for free, bring the group along when you are ready, and pay by the month or the year.",
        "Body/L",
        MUTED,
        json!({"textAlign": "center", "maxWidth": 640}),
    )?;
    let toggle = k.stack(words, "Billing", H, 4.0, json!(4), json!({"fill": SOFT, "cornerRadius": 999}))?;
    k.chip(toggle, "Monthly", ["Label/M", ON_BRAND], PRIMARY, [8.0, 20.0], 999.0)?;
    k.chip(toggle, "Yearly · 2 months off", ["Label/M", INK], SOFT, [8.0, 20.0], 999.0).map(drop)
}

fn plans(k: &mut Kit, root: NodeId) -> Result<()> {
    let row = k.stack(root, "Plans", H, 24.0, json!([16, 96, 72, 96]), json!({"sizingH": "fill", "crossAlign": "center"}))?;
    for (name, price, blurb, features, featured) in PLANS {
        plan(k, row, name, price, blurb, features, featured)?;
    }
    Ok(())
}

fn plan(k: &mut Kit, row: NodeId, name: &str, price: &str, blurb: &str, features: &[&str], featured: bool) -> Result<()> {
    let (fill, ink, soft) = if featured { (DEEP, ON_BRAND, "#CFE6DE") } else { (CARD, INK, MUTED) };
    let pad = if featured { 40 } else { 32 };
    let props = json!({"fill": fill, "cornerRadius": 28, "sizingH": "fill", "effects": shadow(), "stroke": BORDER, "strokeWeight": if featured { 0 } else { 1 }});
    let card = k.stack(row, name, V, 24.0, json!(pad), props)?;
    let top = k.stack(card, "Name", H, 12.0, json!(0), json!({"sizingH": "fill", "mainAlign": "spaceBetween", "crossAlign": "center"}))?;
    k.text(top, name, "Heading/M", ink)?;
    if featured {
        k.chip(top, "Most popular", ["Label/S", INK], ACCENT, [6.0, 12.0], 999.0)?;
    }
    k.para(card, blurb, "Body/M", soft)?;
    let cost = k.stack(card, "Price", H, 8.0, json!(0), json!({"crossAlign": "baseline"}))?;
    k.text_with(cost, price, "Display", ink, json!({}))?;
    k.text(cost, "per month", "Body/M", soft)?;
    let (label_color, button_fill) = if featured { (INK, ACCENT) } else { (ON_BRAND, PRIMARY) };
    let button = k.chip(card, &format!("Choose {name}"), ["Label/M", label_color], button_fill, [14.0, 24.0], 16.0)?;
    k.set(button, json!({"sizingH": "fill"}))?;
    let list = k.stack(card, "Features", V, 0.0, json!(0), json!({"sizingH": "fill"}))?;
    for feature in features {
        let line = json!({"sizingH": "fill", "crossAlign": "center", "stroke": BORDER, "strokeSides": BOTTOM_EDGE, "strokeWeight": 1});
        let line = k.stack(list, feature, H, 12.0, json!([12, 0, 12, 0]), line)?;
        k.icon(line, &glyphs::CHECK, 20.0, if featured { ACCENT } else { PRIMARY })?;
        k.text_with(line, feature, "Body/M", ink, json!({"sizingH": "fill"}))?;
    }
    Ok(())
}

fn questions(k: &mut Kit, root: NodeId) -> Result<()> {
    let section = k.stack(root, "Questions", V, 24.0, json!([72, 240, 72, 240]), json!({"fill": CARD, "sizingH": "fill"}))?;
    k.text(section, "Questions people ask", "Heading/L", INK)?;
    for (question, answer) in QUESTIONS {
        let item = k.stack(
            section,
            question,
            V,
            6.0,
            json!([0, 0, 20, 0]),
            json!({"sizingH": "fill", "stroke": BORDER, "strokeSides": BOTTOM_EDGE, "strokeWeight": 1}),
        )?;
        k.text(item, question, "Heading/S", INK)?;
        k.para(item, answer, "Body/M", MUTED)?;
    }
    Ok(())
}

fn footer(k: &mut Kit, root: NodeId) -> Result<()> {
    let bar = k.stack(
        root,
        "Footer",
        H,
        16.0,
        json!([32, 96, 32, 96]),
        json!({"fill": DEEP, "sizingH": "fill", "mainAlign": "spaceBetween", "crossAlign": "center"}),
    )?;
    let logo = k.stack(bar, "Logo", H, 10.0, json!(0), json!({"crossAlign": "center"}))?;
    k.icon(logo, &glyphs::MOUNTAIN, 24.0, ACCENT)?;
    k.text(logo, "Ridgeline Trails", "Label/M", ON_BRAND)?;
    k.text(bar, "Questions? hello@ridgeline.example", "Body/S", "#CFE6DE").map(drop)
}
