//! Design tokens (the Nodalia Design Language, light and dark, with Trama's accent) and fonts.

use std::sync::{Arc, OnceLock};

use egui::{Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    pub bg: Color32,
    pub panel: Color32,
    pub canvas: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    /// Dim text: disabled controls, placeholders, hints. At least 3:1 against the panels, fields
    /// and selected rows, yet dimmer than `text_secondary`.
    pub text_tertiary: Color32,
    pub icon: Color32,
    pub icon_secondary: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub field: Color32,
    pub field_hover_border: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_on: Color32,
    pub accent_text: Color32,
    pub selected_row: Color32,
    pub selected_row_child: Color32,
    /// Component, variant and instance names and icons in the panels: at least 4.5:1 (WCAG AA
    /// for text) on the panel, on a hovered row and on `component_row`. The canvas keeps
    /// [`COMPONENT`] for its outlines and labels.
    pub component: Color32,
    /// A selected component's row in the Layers panel.
    pub component_row: Color32,
    /// The rows inside a selected component: a tone of its row (`selected_row_child` is the plain
    /// selection's).
    pub component_row_child: Color32,
    pub danger: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub guide: Color32,
    pub tab_bar: Color32,
    pub tab_active: Color32,
    pub shadow: Color32,
    pub menu: Color32,
    pub menu_text: Color32,
    pub menu_hover: Color32,
    pub menu_dim: Color32,
    /// The ring around the control with the keyboard (keyboard navigation): at least 3:1 against
    /// the panels and the canvas.
    pub focus: Color32,
}

/// Trama's purple (its icon's), bright enough for overlays on a dark page (3:1 on the dark canvas).
pub const BRAND: Color32 = Color32::from_rgb(0xA5, 0x5E, 0xEA);
/// The deeper purple for thin selection outlines on light pages and white-on-purple labels: 4.5:1
/// on white.
pub const ACCENT: Color32 = Color32::from_rgb(0x7A, 0x26, 0xC9);
/// Ink on a [`BRAND`] surface.
pub const ACCENT_ON: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
/// Trama's accent in the Nodalia palette.
pub const TRAMA: nodalia_theme::Accent = nodalia_theme::Accent::Trama;
/// Id of the canvas overlays' green for this frame ([`set_canvas_page`]).
const CANVAS_ACCENT: &str = "trama-canvas-accent";

/// Remember the page colour the canvas shows. Its overlays (outlines, handles, arrows) take
/// whichever purple contrasts more with it: the bright one on a dark page, the deeper on a light one.
pub fn set_canvas_page(ctx: &Context, page: Color32) {
    let pick = if c(BRAND).contrast(c(page)) > c(ACCENT).contrast(c(page)) { BRAND } else { ACCENT };
    ctx.data_mut(|d| d.insert_temp(egui::Id::unique(CANVAS_ACCENT), pick));
}

/// `x` (opaque) as a document colour, for its contrast with another.
fn c(x: Color32) -> trama_color::Color {
    trama_color::Color::new(f32::from(x.r()) / 255.0, f32::from(x.g()) / 255.0, f32::from(x.b()) / 255.0, 1.0)
}

/// The purple for overlays drawn on the page ([`set_canvas_page`]); [`ACCENT`] before a page shows.
pub fn canvas_accent(ctx: &Context) -> Color32 {
    ctx.data(|d| d.get_temp(egui::Id::unique(CANVAS_ACCENT))).unwrap_or(ACCENT)
}

/// A fill under white text: the deeper purple stands in for the bright one.
pub fn under_white(c: Color32) -> Color32 {
    if c == BRAND { ACCENT } else { c }
}

/// An AI agent attached over MCP: its avatar and the outline of the layers it works on (none of
/// the people colours, so it never looks like someone in a live session).
pub const AGENT: Color32 = Color32::from_rgb(0xF5, 0x9E, 0x0B);

/// Components, variants and instances on the canvas: magenta, so they never read as the purple
/// accent of a plain selection.
pub const COMPONENT: Color32 = Color32::from_rgb(0xD1, 0x2D, 0x8E);
/// Handoff status badges (white text on these, in both themes).
pub const DEV_READY: Color32 = Color32::from_rgb(0x08, 0x7D, 0x4F);
pub const DEV_CHANGED: Color32 = Color32::from_rgb(0xA7, 0x5B, 0x00);
pub const DEV_COMPLETED: Color32 = Color32::from_rgb(0x6E, 0x6E, 0x73);

impl Tokens {
    /// NDL's light palette with Trama's accent.
    pub fn light() -> Self {
        Self::nodalia(nodalia_theme::Mode::Light)
    }

    /// NDL's dark palette with Trama's accent.
    pub fn dark() -> Self {
        Self::nodalia(nodalia_theme::Mode::Dark)
    }

    /// Every chrome colour from `nodalia-theme` (`Mode::palette_with(Accent::Trama)`). The canvas
    /// marks (components, handoff badges, guides) keep Trama's own, tuned for the pages they sit on.
    fn nodalia(mode: nodalia_theme::Mode) -> Self {
        let p = mode.palette_with(TRAMA);
        let c = ndl;
        let dark = mode.is_dark();
        let panel = c(p.surface_panel);
        let selected = c(p.selection_bg);
        Self {
            dark,
            bg: c(p.surface_window),
            panel,
            canvas: c(p.surface_sunken),
            border: c(p.border_subtle),
            border_strong: c(p.border_default),
            text: c(p.text_primary),
            text_secondary: c(p.text_secondary),
            text_tertiary: c(p.text_tertiary),
            icon: c(p.text_primary),
            icon_secondary: c(p.text_secondary),
            hover: c(p.control_bg_hover),
            pressed: c(p.control_bg_pressed),
            // Trama's fields have no outline, so they take the control fill: NDL's field colour is
            // the panel's own white in the light theme.
            field: c(p.control_bg),
            field_hover_border: c(p.border_default),
            accent: c(p.accent_default),
            accent_hover: c(p.accent_hover),
            accent_on: c(p.text_on_accent),
            accent_text: c(p.accent_text),
            selected_row: selected,
            selected_row_child: mix(panel, selected, 0.5),
            component: if dark { Color32::from_rgb(0xFF, 0x7C, 0xC6) } else { Color32::from_rgb(0xB0, 0x1F, 0x76) },
            component_row: if dark { Color32::from_rgb(0x4E, 0x23, 0x3E) } else { Color32::from_rgb(0xFC, 0xE6, 0xF2) },
            component_row_child: if dark { Color32::from_rgb(0x3D, 0x1E, 0x32) } else { Color32::from_rgb(0xFE, 0xF3, 0xF9) },
            danger: c(p.status_danger_fg),
            success: DEV_READY,
            warning: Color32::from_rgb(0xFF, 0xCD, 0x29),
            guide: Color32::from_rgb(0xF2, 0x48, 0x22),
            tab_bar: c(p.surface_window),
            tab_active: panel,
            shadow: Color32::from_black_alpha(if dark { 90 } else { 40 }),
            menu: Color32::from_rgb(0x1E, 0x1E, 0x1E),
            menu_text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            menu_hover: ACCENT,
            menu_dim: Color32::from_rgb(0x9A, 0x9A, 0x9A),
            focus: c(p.border_focus),
        }
    }

    /// The creator suite's shared parts (`nodalia_theme::creator`: workspace selector, Home,
    /// dialog buttons) in these tokens' palette.
    pub fn creator(&self) -> nodalia_theme::creator::Look {
        let mode = if self.dark { nodalia_theme::Mode::Dark } else { nodalia_theme::Mode::Light };
        nodalia_theme::creator::Look::new(mode.palette_with(TRAMA))
            .strong_family(FontFamily::Name("semibold".into()))
            .medium_family(FontFamily::Name("semibold".into()))
    }

    pub fn get(ctx: &Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(tokens_id())).unwrap_or_else(Tokens::light)
    }
}

/// The window-coloured gap between the panels' islands, and from them to the window's edges.
pub const ISLAND_GAP: f32 = 8.0;
/// An island's corner radius (NDL's `ISLAND_RADIUS`).
pub const ISLAND_RADIUS: u8 = nodalia_theme::creator::layout::ISLAND_RADIUS;

/// Rounds the corners of the island `rect` after its contents are drawn: a window-coloured ring
/// outside the rounded rect, clipped to it.
pub fn round_island(painter: &egui::Painter, t: &Tokens, rect: egui::Rect) {
    nodalia_theme::creator::round_island_corners(painter, &t.creator(), rect);
}

/// Stable id for temporary theme tokens.
fn tokens_id() -> egui::Id {
    egui::Id::unique("trama_tokens")
}

/// egui's default for how long hover fades and openings animate.
const ANIMATION_TIME: f32 = 0.2;

/// Install tokens and egui visuals.
pub fn apply(ctx: &Context, t: Tokens) {
    apply_with(ctx, t, false);
}

/// [`apply`], with egui's own animations (hover fades, openings, smooth scrolling to a target) off
/// when the system or the preferences ask for reduced motion. Prototype transitions still play.
pub fn apply_with(ctx: &Context, t: Tokens, reduce_motion: bool) {
    ctx.data_mut(|d| d.insert_temp(tokens_id(), t));
    ctx.set_visuals(visuals(t));
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(8.0, 6.0);
        s.spacing.button_padding = egui::vec2(8.0, 4.0);
        s.spacing.interact_size = egui::vec2(24.0, 24.0);
        s.spacing.menu_margin = egui::Margin::same(8);
        s.spacing.window_margin = egui::Margin::same(12);
        s.text_styles = [
            (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(11.0, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(11.0, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(13.0, FontFamily::Name("semibold".into()))),
            (TextStyle::Monospace, FontId::new(11.0, FontFamily::Monospace)),
        ]
        .into();
        s.interaction.tooltip_delay = 0.4;
        s.animation_time = if reduce_motion { 0.0 } else { ANIMATION_TIME };
        s.scroll_animation = if reduce_motion { egui::style::ScrollAnimation::none() } else { egui::style::ScrollAnimation::default() };
    });
}

/// egui visuals for tokens `t`.
fn visuals(t: Tokens) -> Visuals {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.hover;
    v.override_text_color = Some(t.text);
    v.window_stroke = Stroke::new(1.0, t.border);
    v.window_corner_radius = egui::CornerRadius::same(13);
    v.menu_corner_radius = egui::CornerRadius::same(13);
    v.selection.bg_fill = t.accent.gamma_multiply(0.3);
    // Also the colour of selected text on that highlight (and a focused text box's outline): the
    // theme's readable green, not the darker one that sinks into the dark theme's highlight.
    v.selection.stroke = Stroke::new(1.0, t.accent_text);
    v.hyperlink_color = t.accent_text;
    for w in [&mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open, &mut v.widgets.noninteractive] {
        w.corner_radius = egui::CornerRadius::same(5);
        w.fg_stroke = Stroke::new(1.0, t.text);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.border);
    v.widgets.noninteractive.bg_fill = t.panel;
    v.widgets.inactive.bg_fill = t.field;
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.field_hover_border);
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    v.popup_shadow = egui::Shadow { offset: [0, 4], blur: 16, spread: 0, color: t.shadow };
    v.window_shadow = egui::Shadow { offset: [0, 8], blur: 24, spread: 0, color: t.shadow };
    v.text_cursor.stroke = Stroke::new(1.0, ACCENT);
    v
}

/// Draw `f` with the dark tokens and visuals: controls on the dark popovers (the style browser's
/// style editor), whichever theme the app shows.
pub fn on_dark<R>(ui: &mut egui::Ui, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let ctx = ui.ctx().clone();
    let saved = Tokens::get(&ctx);
    let dark = Tokens::dark();
    ctx.data_mut(|d| d.insert_temp(tokens_id(), dark));
    let r = ui
        .scope(|ui| {
            *ui.visuals_mut() = visuals(dark);
            f(ui)
        })
        .inner;
    ctx.data_mut(|d| d.insert_temp(tokens_id(), saved));
    r
}

/// UI fonts: Inter and the Archivo wordmark (from the craft-fonts build input) when present;
/// otherwise egui's defaults. With
/// `system`, also system faces for scripts those lack (Hebrew, joined Arabic, Ethiopic, the Indic
/// scripts and the others [`crate::bidi::script_fallbacks`] names) so names in them show,
/// and faces for the CJK scripts in `cjk` ([`crate::cjk`]).
///
/// The craft-fonts set embeds Noto Sans Arabic and Noto Sans Hebrew; with `system` off those
/// built-in faces are the whole RTL fallback, so tests and releases do not depend on installed fonts.
pub fn install_fonts(ctx: &Context, system: bool, cjk: u8) {
    let mut fonts = FontDefinitions::default();
    let find = |family: &str, style: &str| trama_text::CRAFT_FONTS.iter().find(|f| f.family == family && f.style == style).map(|f| f.bytes);
    let regular = find("Inter", "Regular");
    let semibold = find("Inter", "SemiBold").or_else(|| find("Inter", "Bold"));
    if let Some(r) = regular {
        fonts.font_data.insert("inter".into(), Arc::new(FontData::from_static(r)));
        if let Some(list) = fonts.families.get_mut(&FontFamily::Proportional) {
            list.insert(0, "inter".into());
        }
    }
    let sb_name = if let Some(sb) = semibold {
        fonts.font_data.insert("inter-semibold".into(), Arc::new(FontData::from_static(sb)));
        "inter-semibold"
    } else {
        "inter"
    };
    let mut semi: Vec<String> = Vec::new();
    if fonts.font_data.contains_key(sb_name) {
        semi.push(sb_name.into());
    }
    semi.extend(fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    fonts.families.insert(FontFamily::Name("semibold".into()), semi);
    // Keep the logo's typeface independent of UI text. System Archivo is used when available;
    // releases with craft-fonts embed it so the wordmark does not depend on installation.
    let archivo = find("Archivo", "SemiBold").map(FontData::from_static).or_else(|| {
        system.then(|| trama_text::find_face("Archivo", "SemiBold")).flatten().filter(|face| face.family == "Archivo").map(|face| font_data(&face))
    });
    let mut brand = Vec::new();
    if let Some(data) = archivo {
        fonts.font_data.insert("archivo".into(), Arc::new(data));
        brand.push("archivo".into());
    }
    brand.extend(fonts.families.get(&FontFamily::Name("semibold".into())).cloned().unwrap_or_default());
    fonts.families.insert(FontFamily::Name("brand".into()), brand);
    // CJK and other scripts from craft-fonts as fallbacks (Noto Sans CJK only with TRAMA_EMBED_CJK).
    const BUILT_IN: [&str; 7] =
        ["Inter", "BIZ UDPGothic", "Noto Sans CJK SC", "Noto Sans Arabic", "Noto Sans Hebrew", "Noto Sans Georgian", "Noto Sans Thai"];
    let lang = crate::i18n::current().map_or("en", |l| l.code);
    // A Chinese UI draws Han in a Chinese face: the built-in Japanese one goes after the system faces.
    let late = |family: &str| family == "BIZ UDPGothic" && crate::cjk::language(lang) == crate::cjk::HAN;
    let craft = |last: bool| {
        trama_text::CRAFT_FONTS
            .iter()
            .filter(move |f| f.family != "Inter" && f.style == "Regular" && BUILT_IN.contains(&f.family) && late(f.family) == last)
            .map(|f| (format!("craft-{}", f.family), FontData::from_static(f.bytes)))
    };
    let mut extra: Vec<(String, FontData)> = craft(false).collect();
    if system {
        // What the built-in faces above already draw.
        let embedded = |c: char| {
            BUILT_IN
                .into_iter()
                .filter(|f| !late(f) && trama_text::CRAFT_FONTS.iter().any(|cf| cf.family == *f))
                .filter_map(|f| trama_text::find_face(f, "Regular"))
                .any(|x| x.has_char(c))
        };
        let mut taken = Vec::new();
        extra.extend(crate::bidi::script_fallbacks(&embedded, &mut taken));
        extra.extend(key_symbol_fallbacks(&embedded, &mut taken));
        extra.extend(crate::cjk::faces(cjk, lang, &embedded).iter().map(|f| (format!("system-{}-{}", f.family, f.style), font_data(f))));
    }
    extra.extend(craft(true));
    for (key, data) in extra {
        fonts.font_data.insert(key.clone(), Arc::new(data));
        for fam in [FontFamily::Proportional, FontFamily::Name("semibold".into())] {
            if let Some(list) = fonts.families.get_mut(&fam) {
                list.push(key.clone());
            }
        }
    }
    ctx.set_fonts(fonts);
}

/// The glyphs macOS shortcuts are written with ([`trama_engine::keymap::display`]).
const KEY_SYMBOLS: &str = "⌃⌥⇧⌘⌫↵⇥⇞⇟←→↑↓";
/// System families that have them, in order of preference.
const KEY_FAMILIES: [&str; 6] = ["Lucida Grande", "Apple Symbols", "Segoe UI Symbol", "DejaVu Sans", "Noto Sans Symbols 2", "FreeSans"];

/// System faces for the shortcut glyphs the UI's own fonts lack (egui's default face has ⌘ but
/// not ⌥ or ↵); `taken` as for [`system_fallbacks`].
fn key_symbol_fallbacks(have: &dyn Fn(char) -> bool, taken: &mut Vec<Arc<trama_text::Face>>) -> Vec<(String, FontData)> {
    system_fallbacks(KEY_SYMBOLS.chars(), &KEY_FAMILIES, have, taken)
}

/// System faces, the first of `families` that has each, for the `samples` neither the UI's own
/// fonts (`have`) nor the faces already `taken` draw. Each face found joins `taken`, so a face that
/// several lists name is added once.
pub(crate) fn system_fallbacks(
    samples: impl IntoIterator<Item = char>,
    families: &[&str],
    have: &dyn Fn(char) -> bool,
    taken: &mut Vec<Arc<trama_text::Face>>,
) -> Vec<(String, FontData)> {
    let mut out = Vec::new();
    for c in samples {
        if have(c) || taken.iter().any(|f| f.has_char(c)) {
            continue;
        }
        let Some(face) = trama_text::face_with_char(c, families) else { continue };
        out.push((format!("system-{}-{}", face.family, face.style), font_data(&face)));
        taken.push(face);
    }
    out
}

/// egui font data over a face's bytes, without copying them (CJK faces are 10–20 MB). Font
/// database faces are never dropped, so a system face's shared bytes are kept for good: leaking
/// one more reference to them costs a pointer and gives egui the `'static` slice it needs.
pub fn font_data(face: &trama_text::Face) -> FontData {
    let bytes: &'static [u8] = match &face.bytes {
        trama_text::fontdb::FontBytes::Static(b) => b,
        trama_text::fontdb::FontBytes::Shared(v) => Box::leak(Box::new(v.clone())).as_slice(),
    };
    let mut data = FontData::from_static(bytes);
    data.index = face.index;
    data
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("semibold".into()))
}

pub fn brand(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("brand".into()))
}

/// The wordmark used in the home sidebar and About window: "Trama" in the brand face.
pub fn paint_wordmark(painter: &egui::Painter, left_center: egui::Pos2, size: f32, t: &Tokens) {
    painter.text(left_center, egui::Align2::LEFT_CENTER, "Trama", brand(size), t.text);
}

/// Paint Trama's app icon (Babbage's portrait) from its 64 px render.
pub fn paint_app_icon(ui: &egui::Ui, rect: egui::Rect) {
    static ICON: OnceLock<Option<egui::ColorImage>> = OnceLock::new();
    let image = ICON.get_or_init(|| {
        let rgba = image::load_from_memory(include_bytes!("../../../assets/app-icon/trama-64.png")).ok()?.to_rgba8();
        Some(egui::ColorImage::from_rgba_unmultiplied([rgba.width() as usize, rgba.height() as usize], rgba.as_raw()))
    });
    let Some(image) = image else {
        ui.painter().rect_filled(rect, egui::CornerRadius::same(5), BRAND);
        crate::icons::paint(ui.painter(), rect.shrink(rect.width() * 0.22), crate::icons::Icon::Logo, ACCENT_ON);
        return;
    };
    let id = egui::Id::unique("trama_original_app_icon");
    let texture = ui.ctx().data(|data| data.get_temp::<egui::TextureHandle>(id)).unwrap_or_else(|| {
        let texture = ui.ctx().load_texture("trama-original-app-icon", image.clone(), egui::TextureOptions::LINEAR);
        ui.ctx().data_mut(|data| data.insert_temp(id, texture.clone()));
        texture
    });
    ui.painter().image(texture.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
}

pub fn regular(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn c32(c: trama_color::Color) -> Color32 {
    let [r, g, b, a] = c.to_rgba8();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Stable id for the current document-to-screen colour conversion.
fn screen_conversion_id() -> egui::Id {
    egui::Id::unique("trama_screen_conversion")
}

/// Remember the colour profile of the document on screen (once per frame): document colours drawn
/// by the UI ([`doc_c32`]) go from it to the screen's sRGB.
pub fn set_doc_profile(ctx: &Context, profile: trama_color::ColorProfile) {
    let conv = profile.conversion_to(trama_color::ColorProfile::Srgb);
    ctx.data_mut(|d| d.insert_temp(screen_conversion_id(), conv));
}

/// A document colour (a paint, swatch, background) for the screen: converted from the document's
/// colour profile to sRGB. UI chrome colours use [`c32`].
pub fn doc_c32(ctx: &Context, c: trama_color::Color) -> Color32 {
    let conv = ctx.data(|d| d.get_temp::<trama_color::ProfileConversion>(screen_conversion_id()));
    c32(conv.map_or(c, |k| k.apply(c)))
}

/// An NDL colour as egui's.
pub(crate) fn ndl(c: nodalia_theme::Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

/// `a` moved `k` of the way to `b`.
fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let m = |p: u8, q: u8| (f32::from(p) + (f32::from(q) - f32::from(p)) * k).round() as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_overlays_take_the_green_that_stands_out_on_the_page() {
        let ctx = Context::default();
        assert_eq!(canvas_accent(&ctx), ACCENT, "before any page shows");
        set_canvas_page(&ctx, Color32::from_rgb(0x1E, 0x1E, 0x1E));
        assert_eq!(canvas_accent(&ctx), BRAND, "the bright green on a dark page");
        set_canvas_page(&ctx, Color32::WHITE);
        assert_eq!(canvas_accent(&ctx), ACCENT, "the darker green on a light page");
        set_canvas_page(&ctx, Color32::from_rgb(0xF5, 0xF5, 0xF5));
        assert_eq!(canvas_accent(&ctx), ACCENT);
        // White text never sits on the bright green.
        assert_eq!(under_white(BRAND), ACCENT);
        assert_eq!(under_white(COMPONENT), COMPONENT);
    }

    /// `fill` (premultiplied, maybe translucent) painted over the opaque `bg`.
    fn over(fill: Color32, bg: Color32) -> Color32 {
        let k = 1.0 - f32::from(fill.a()) / 255.0;
        let mix = |f: u8, b: u8| (f32::from(f) + f32::from(b) * k).round().min(255.0) as u8;
        Color32::from_rgb(mix(fill.r(), bg.r()), mix(fill.g(), bg.g()), mix(fill.b(), bg.b()))
    }

    #[test]
    fn disabled_and_selected_text_stay_readable_in_both_themes() {
        for t in [Tokens::light(), Tokens::dark()] {
            let theme = if t.dark { "dark" } else { "light" };
            // Disabled text (a disabled button's label, placeholders) on panels, fields and
            // selected rows, still dimmer than secondary text.
            for bg in [t.panel, t.field, t.hover, t.selected_row, t.selected_row_child, t.component_row, t.component_row_child] {
                let ratio = c(t.text_tertiary).contrast(c(bg));
                assert!(ratio >= 3.0, "{theme}: dim text {:?} on {bg:?} is {ratio:.2}:1", t.text_tertiary);
                assert!(ratio < c(t.text_secondary).contrast(c(bg)), "{theme}: dim text reads dimmer than secondary text");
            }
            // Selected text (renaming a page or layer, any text box) on the selection highlight.
            let v = visuals(t);
            for bg in [t.panel, t.field] {
                let highlight = over(v.selection.bg_fill, bg);
                let ratio = c(v.selection.stroke.color).contrast(c(highlight));
                assert!(ratio >= 3.0, "{theme}: selected text on {highlight:?} is {ratio:.2}:1");
            }
        }
    }

    #[test]
    fn ui_font_draws_every_mac_shortcut_glyph() {
        trama_text::scan_system_fonts();
        let lacking: String = KEY_SYMBOLS.chars().filter(|c| trama_text::face_with_char(*c, &KEY_FAMILIES).is_none()).collect();
        if !lacking.is_empty() {
            eprintln!("no system face here draws {lacking}; skipping");
            return;
        }
        let ctx = Context::default();
        install_fonts(&ctx, true, 0);
        let input =
            egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100.0, 100.0))), ..Default::default() };
        ctx.run_ui(input, |_| {}).textures_delta.clear();
        ctx.fonts_mut(|f| {
            for c in KEY_SYMBOLS.chars() {
                for font in [regular(11.0), semibold(11.0)] {
                    assert!(f.has_glyphs(&font, &c.to_string()), "UI font draws {c:?}");
                }
            }
        });
    }

    #[test]
    fn ui_font_draws_thai_with_the_built_in_thai_face() {
        // The web build has no system fonts: Thai layer names, frame labels and field text are drawn
        // by craft-fonts' Noto Sans Thai, a built-in fallback, when the build embeds it.
        if !trama_text::CRAFT_FONTS.iter().any(|f| f.family == "Noto Sans Thai") {
            eprintln!("built without craft-fonts' Noto Sans Thai; skipping");
            return;
        }
        let ctx = Context::default();
        install_fonts(&ctx, false, 0);
        let input =
            egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100.0, 100.0))), ..Default::default() };
        ctx.run_ui(input, |_| {}).textures_delta.clear();
        // "ภาษาไทย" (the Thai language) and "น้ำ" (water: a tone mark over SARA AM).
        let thai = "\u{0E20}\u{0E32}\u{0E29}\u{0E32}\u{0E44}\u{0E17}\u{0E22} \u{0E19}\u{0E49}\u{0E33}";
        ctx.fonts_mut(|f| {
            for font in [regular(11.0), semibold(11.0)] {
                assert!(f.has_glyphs(&font, thai), "UI font draws {thai:?}");
            }
        });
    }
}
