//! Controls: buttons, chips, avatars, toggles, segmented controls and text fields.
//!
//! Builders end in `.build(&theme)` and return a plain [`El`], so handlers attach the Rune way:
//! `Button::new("Save").build(&t).on_press(|s: &mut App| s.save())`.
use rune::{Color, El, Icon, Shadow, TextEdit, TextStyle, Tx, col, el, icon, oklch, row, text, text_input};

use crate::surface::{Level, paint};
use crate::theme::{Skin, Theme};

/// Semantic colour of a chip, pill or badge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
    Info,
}

impl Tone {
    pub fn color(self, t: &Theme) -> Color {
        match self {
            Tone::Neutral => t.c.muted,
            Tone::Accent => t.c.accent,
            Tone::Success => t.c.success,
            Tone::Warning => t.c.warning,
            Tone::Danger => t.c.danger,
            Tone::Info => t.c.info,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Kind {
    #[default]
    Primary,
    /// Tonal or glass button for secondary actions.
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    fn height(self) -> f32 {
        match self {
            Size::Small => 30.0,
            Size::Medium => 38.0,
            Size::Large => 46.0,
        }
    }
}

fn hard(t: &Theme, off: f32) -> Shadow {
    Shadow { x: off, y: off, blur: 0.0, spread: 0.0, color: t.c.shadow, inset: false }
}

fn glow(color: Color, y: f32, blur: f32) -> Shadow {
    Shadow { x: 0.0, y, blur, spread: -2.0, color, inset: false }
}

/// Paints a filled control (button face, active segment, checked box) in `fill`.
pub(crate) fn solid(t: &Theme, e: El, fill: Color, radius: f32) -> El {
    let e = e.radius(radius).fill(fill);
    match t.skin {
        Skin::Liquid => e
            .linear_gradient(std::f32::consts::PI, [(0.0, Color::WHITE.alpha(0.38)), (0.5, Color::WHITE.alpha(0.0)), (1.0, Color::BLACK.alpha(0.08))])
            .border_gradient(1.0, Color::WHITE.alpha(0.75), Color::WHITE.alpha(0.1))
            .box_shadow(glow(fill.alpha(0.45), 6.0, 18.0)),
        Skin::Frost => e.border(1.0, Color::WHITE.alpha(0.35)).box_shadow(glow(fill.alpha(0.3), 4.0, 12.0)),
        Skin::Material => e,
        Skin::Bold => e.border(t.stroke, t.c.stroke).box_shadow(hard(t, 3.0)),
    }
}

pub struct Button {
    label: String,
    kind: Kind,
    size: Size,
    icon: Option<&'static Icon>,
    trailing: Option<&'static Icon>,
    full: bool,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Button {
        Button { label: label.into(), kind: Kind::Primary, size: Size::Medium, icon: None, trailing: None, full: false }
    }
    pub fn kind(mut self, k: Kind) -> Self {
        self.kind = k;
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn icon(mut self, i: &'static Icon) -> Self {
        self.icon = Some(i);
        self
    }
    pub fn trailing_icon(mut self, i: &'static Icon) -> Self {
        self.trailing = Some(i);
        self
    }
    /// Stretch to the container's width.
    pub fn full_width(mut self) -> Self {
        self.full = true;
        self
    }
    pub fn build(self, t: &Theme) -> El {
        let c = &t.c;
        let radius = match t.skin {
            Skin::Material | Skin::Liquid => t.radius.pill,
            _ => t.radius.sm,
        };
        let h = self.size.height();
        let font = match self.size {
            Size::Small => t.ty.caption,
            _ => t.ty.label,
        };
        let (ink, face) = match self.kind {
            Kind::Primary => (if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) }, Some(c.accent)),
            Kind::Danger => (t.ink_on(c.danger), Some(c.danger)),
            Kind::Secondary => (c.text, None),
            Kind::Outline => (c.text, None),
            Kind::Ghost => (c.muted, None),
        };
        let mut e = row().h(h).px(if self.size == Size::Small { 12.0 } else { 16.0 }).gap(7.0).center()
            .role("button").label(self.label.clone()).focusable(true).press_scale(0.97).motion(t.spring);
        if self.full {
            e = e.w_full();
        }
        e = match (self.kind, face) {
            (_, Some(f)) => solid(t, e, f, radius),
            (Kind::Secondary, _) => match t.skin {
                Skin::Material => e.radius(radius).fill(c.accent_soft),
                Skin::Bold => e.radius(radius).fill(c.surface_alt).border(t.stroke, c.stroke).box_shadow(hard(t, 3.0)),
                _ => paint(t, e, Level::Card, radius, false),
            },
            (Kind::Outline, _) => e.radius(radius).border(if t.skin == Skin::Bold { t.stroke } else { 1.25 }, if t.skin == Skin::Bold { c.stroke } else { c.line.alpha(1.0).alpha(0.9) }),
            _ => e.radius(radius),
        };
        e = e.hover(c.hover).press(c.press);
        if let Some(i) = self.icon {
            e = e.child(icon(i, 15.0).color(ink));
        }
        if !self.label.is_empty() {
            e = e.child(text(self.label).font(font).color(ink));
        }
        if let Some(i) = self.trailing {
            e = e.child(icon(i, 14.0).color(ink));
        }
        e
    }
}

/// A square (or round) button showing only an icon.
pub fn icon_button(t: &Theme, i: &'static Icon, label: &str, active: bool) -> El {
    let c = &t.c;
    let r = if matches!(t.skin, Skin::Liquid | Skin::Material) { 19.0 } else { t.radius.sm };
    let e = row().size(38.0, 38.0).center().role("button").label(label.to_owned()).focusable(true).press_scale(0.94);
    let e = if active {
        solid(t, e, c.accent, r)
    } else if t.skin == Skin::Bold {
        e.radius(r).fill(c.surface).border(t.stroke, c.stroke)
    } else {
        e.radius(r)
    };
    e.hover(c.hover).press(c.press).child(icon(i, 17.0).color(if active { t.ink_on(c.accent) } else { c.muted }))
}

/// A small rounded label in a tone, optionally with a leading dot.
pub fn chip(t: &Theme, label: &str, tone: Tone) -> El {
    let col = tone.color(t);
    let base = text(label.to_owned()).font(t.ty.caption).px(9.0).py(3.0).radius(t.radius.pill.min(10.0));
    match t.skin {
        Skin::Bold => base.color(t.c.text).fill(if tone == Tone::Neutral { t.c.surface_alt } else { col }).border(2.0, t.c.stroke),
        Skin::Material => base.color(if tone == Tone::Neutral { t.c.muted } else { col }).fill(col.alpha(0.14)),
        _ => base.color(if t.dark { col } else { oklch_darker(col) }).fill(col.alpha(if t.dark { 0.18 } else { 0.16 })).border(1.0, col.alpha(0.35)),
    }
}

fn oklch_darker(c: Color) -> Color {
    Color { r: c.r * 0.72, g: c.g * 0.72, b: c.b * 0.72, a: c.a }
}

/// A status pill: dot plus label ("Paid", "Shipped", "Overdue").
pub fn status(t: &Theme, label: &str, tone: Tone) -> El {
    let col = tone.color(t);
    let dot = el().size(7.0, 7.0).radius(3.5).fill(col);
    let dot = if t.skin == Skin::Bold { dot.border(1.5, t.c.stroke).size(9.0, 9.0).radius(4.5) } else { dot };
    row().gap(6.0).center_y().child(dot).child(text(label.to_owned()).font(t.ty.caption).color(t.c.muted))
}

/// A counter badge ("3", "99+") in the danger colour.
pub fn badge(t: &Theme, n: u32) -> El {
    let ink = if t.skin == Skin::Bold { t.c.text } else { Color::WHITE };
    badge_in(t, n, t.c.danger, ink)
}

/// A counter badge in any fill and ink.
pub fn badge_in(t: &Theme, n: u32, fill: Color, ink: Color) -> El {
    let label = if n > 99 { "99+".to_string() } else { n.to_string() };
    let e = row().h(18.0).min_w(18.0).px(5.0).center().radius(9.0).fill(fill)
        .child(text(label).font(TextStyle::new(11.0, 700.0)).color(ink));
    if t.skin == Skin::Bold { e.border(2.0, t.c.stroke).h(20.0).min_w(20.0).radius(10.0) } else { e }
}

/// A round avatar with initials, coloured from the name.
pub fn avatar(t: &Theme, name: &str, size: f32) -> El {
    let hue = name.bytes().fold(7u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32)) % 360;
    let initials: String = name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect::<String>().to_uppercase();
    let fill = if t.skin == Skin::Bold { oklch(0.82, 0.14, hue as f32) } else { oklch(0.66, 0.13, hue as f32) };
    let ink = if t.skin == Skin::Bold { t.c.text } else { Color::WHITE };
    let mut e = row().size(size, size).radius(size / 2.0).center().fill(fill)
        .child(text(initials).font(TextStyle::new((size * 0.38).max(9.0), 650.0)).color(ink));
    e = match t.skin {
        Skin::Bold => e.border(2.0, t.c.stroke),
        Skin::Liquid | Skin::Frost => e.border(1.5, Color::WHITE.alpha(if t.dark { 0.35 } else { 0.8 })),
        Skin::Material => e,
    };
    e
}

/// Overlapping avatars with a "+n" tail.
pub fn avatar_stack(t: &Theme, names: &[&str], size: f32, max: usize) -> El {
    let mut r = row();
    for (i, n) in names.iter().take(max).enumerate() {
        r = r.child(avatar(t, n, size).ml(if i == 0 { 0.0 } else { -size * 0.3 }));
    }
    if names.len() > max {
        r = r.child(row().size(size, size).radius(size / 2.0).center().ml(-size * 0.3).fill(t.c.surface_alt)
            .border(if t.skin == Skin::Bold { 2.0 } else { 1.0 }, t.c.stroke)
            .child(text(format!("+{}", names.len() - max)).font(TextStyle::new(size * 0.34, 650.0)).color(t.c.muted)));
    }
    r
}

/// An on/off switch. Attach `.on_press` to the result to toggle.
pub fn switch(t: &Theme, on: bool) -> El {
    let c = &t.c;
    let (w, h) = if t.skin == Skin::Material { (52.0, 32.0) } else { (46.0, 28.0) };
    let pad = if t.skin == Skin::Bold { 3.0 } else { 3.0 };
    let k = h - pad * 2.0 - if t.skin == Skin::Bold { 2.0 * t.stroke - 2.0 } else { 0.0 };
    let knob = el().key("knob").size(k, k).radius(k / 2.0).motion(t.spring)
        .absolute(if on { w - k - pad - if t.skin == Skin::Bold { t.stroke * 2.0 - 2.0 } else { 0.0 } } else { pad }, pad - if t.skin == Skin::Bold { 1.0 } else { 0.0 });
    let knob = match t.skin {
        Skin::Bold => knob.fill(if on { c.text } else { c.surface }).border(2.0, c.stroke),
        Skin::Material => knob.fill(if on { c.on_accent } else { c.stroke }),
        _ => knob.fill(Color::WHITE).box_shadow(Shadow { x: 0.0, y: 2.0, blur: 6.0, spread: 0.0, color: Color::BLACK.alpha(0.22), inset: false }),
    };
    let track = el().size(w, h).role("switch").checked(on).focusable(true).motion(t.spring);
    let track = if on {
        solid(t, track, c.accent, h / 2.0).box_shadow(Shadow { x: 0.0, y: 0.0, blur: 0.0, spread: 0.0, color: Color::TRANSPARENT, inset: false })
    } else {
        match t.skin {
            Skin::Bold => track.radius(h / 2.0).fill(c.surface_alt).border(t.stroke, c.stroke),
            Skin::Material => track.radius(h / 2.0).fill(c.surface_alt).border(2.0, c.stroke),
            _ => paint(t, track, Level::Inset, h / 2.0, false),
        }
    };
    track.child(knob)
}

/// A checkbox. Attach `.on_press` to toggle.
pub fn checkbox(t: &Theme, on: bool) -> El {
    let c = &t.c;
    let s = 20.0;
    let r = if t.skin == Skin::Bold { 4.0 } else { 6.0 };
    let e = row().size(s, s).center().role("checkbox").checked(on).focusable(true).press_scale(0.9).motion(t.spring);
    if on {
        solid(t, e, c.accent, r).box_shadow(Shadow { x: 0.0, y: 0.0, blur: 0.0, spread: 0.0, color: Color::TRANSPARENT, inset: false })
            .child(icon(&crate::icons::CHECK, 14.0).color(if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) }))
    } else {
        e.radius(r).border(if t.skin == Skin::Bold { 2.0 } else { 1.5 }, if t.skin == Skin::Bold { c.stroke } else { c.faint }).fill(c.surface_alt)
    }
}

/// A segmented control. `on_pick(i)` receives the chosen index.
pub fn segmented<S: 'static>(t: &Theme, labels: &[&str], selected: usize, on_pick: impl Fn(&mut S, usize) + Clone + 'static) -> El {
    let c = &t.c;
    let outer_r = if matches!(t.skin, Skin::Liquid | Skin::Material) { 19.0 } else { t.radius.sm + 3.0 };
    let mut r = row().h(38.0).padding(3.0).gap(2.0).role("tablist");
    r = match t.skin {
        Skin::Bold => r.radius(outer_r).fill(c.surface).border(t.stroke, c.stroke),
        Skin::Material => r.radius(outer_r).border(1.0, c.stroke),
        _ => paint(t, r, Level::Inset, outer_r, false),
    };
    for (i, l) in labels.iter().enumerate() {
        let on = i == selected;
        let f = on_pick.clone();
        let mut seg = row().h_full().px(14.0).center().role("tab").checked(on).label((*l).to_owned()).focusable(true).motion(t.spring)
            .on_press(move |s: &mut S| f(s, i));
        seg = if on {
            match t.skin {
                Skin::Material => seg.radius(outer_r - 3.0).fill(c.accent_soft),
                Skin::Bold => seg.radius(t.radius.sm - 1.0).fill(c.accent).border(2.0, c.stroke),
                _ => paint(t, seg, Level::Float, outer_r - 3.0, false),
            }
        } else {
            seg.radius(outer_r - 3.0).hover(c.hover)
        };
        let ink = if on { if t.skin == Skin::Bold { c.on_accent } else { c.text } } else { c.muted };
        r = r.child(seg.child(text((*l).to_owned()).font(t.ty.label).color(ink)));
    }
    r
}

/// A text field with an optional leading icon. `edit` gives the field's state, `access` reaches it
/// from the app state, and `on_submit` runs on Return.
pub fn text_field<S: 'static>(
    t: &Theme,
    edit: &TextEdit,
    placeholder: &str,
    lead: Option<&'static Icon>,
    access: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static,
    on_submit: impl Fn(&mut S, &mut Tx) + 'static,
) -> El {
    let c = &t.c;
    let r = if matches!(t.skin, Skin::Liquid) { t.radius.pill } else { t.radius.sm };
    let input = text_input(edit, t.ty.body).single_line().placeholder(placeholder.to_owned(), c.faint).color(c.text)
        .grow(1.0).h(40.0).py(10.0).on_edit(access).on_submit(on_submit);
    let mut w = row().h(42.0).px(12.0).gap(8.0).center_y();
    w = match t.skin {
        Skin::Bold => w.radius(r).fill(c.surface).border(t.stroke, if edit.focused { c.accent } else { c.stroke }),
        Skin::Material => w.radius(r).fill(c.surface_alt).border_sides([0.0, 0.0, if edit.focused { 2.0 } else { 1.0 }, 0.0], if edit.focused { c.accent } else { c.stroke }).corners([r, r, 0.0, 0.0]),
        _ => paint(t, w, Level::Inset, r, false),
    };
    if let Some(i) = lead {
        w = w.child(icon(i, 16.0).color(c.faint));
    }
    w.child(input)
}

/// A horizontal progress bar, `value` in 0..=1.
pub fn progress(t: &Theme, value: f32, tone: Tone) -> El {
    let v = value.clamp(0.0, 1.0);
    let h = if t.skin == Skin::Bold { 12.0 } else { 8.0 };
    let col = if tone == Tone::Neutral { t.c.accent } else { tone.color(t) };
    let track = row().w_full().h(h).radius(h / 2.0);
    let track = match t.skin {
        Skin::Bold => track.fill(t.c.surface).border(2.0, t.c.stroke),
        Skin::Material => track.fill(t.c.surface_alt),
        _ => paint(t, track, Level::Inset, h / 2.0, false),
    };
    let bar = el().h_full().radius(h / 2.0).fill(col).motion(t.spring);
    let bar = if t.skin == Skin::Liquid { bar.gradient(Color::WHITE.alpha(0.35).blend_into(col), col) } else { bar };
    track.child(col_w(bar, v)).child(el().grow((1.0 - v).max(0.0001)))
}

fn col_w(bar: El, v: f32) -> El {
    col().grow(v.max(0.0001)).h_full().child(bar.w_full())
}

trait Blend {
    fn blend_into(self, base: Color) -> Color;
}

impl Blend for Color {
    fn blend_into(self, base: Color) -> Color {
        let a = self.a;
        Color { r: base.r + (self.r - base.r) * a, g: base.g + (self.g - base.g) * a, b: base.b + (self.b - base.b) * a, a: base.a }
    }
}

/// Small "key: value" pair, label on top.
pub fn field(t: &Theme, label: &str, value: El) -> El {
    col().gap(4.0).child(text(label.to_owned()).font(t.ty.caption).color(t.c.faint)).child(value)
}

#[allow(dead_code)]
fn unused(_: f32) -> Color {
    oklch(0.5, 0.0, 0.0)
}
