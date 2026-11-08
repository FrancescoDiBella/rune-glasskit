//! How a pane looks in each skin. Every component paints its container through [`paint`], so
//! changing the skin restyles the whole app.
use rune::{Color, El, Shadow, el, oklcha};

use crate::theme::{Skin, Theme};

/// How far a surface sits above the page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    /// Wells: inputs, table headers, tracks. Sunk into the surface around them.
    Inset,
    /// Cards and list containers.
    Card,
    /// Large panes: sidebars, top bars, boards.
    Pane,
    /// Floating things: toasts, menus, the chat composer.
    Float,
}

fn shadow(x: f32, y: f32, blur: f32, spread: f32, color: Color, inset: bool) -> Shadow {
    Shadow { x, y, blur, spread, color, inset }
}

/// Paints `e` as a surface at `level` with `radius`. `blur` asks for a real backdrop blur on glass
/// skins (Rune blurs at most four elements a frame, so reserve it for large panes); without it the
/// pane is drawn with the same tint, rim and depth.
pub fn paint(t: &Theme, e: El, level: Level, radius: f32, blur: bool) -> El {
    let c = &t.c;
    let e = e.radius(radius);
    match t.skin {
        Skin::Liquid => {
            let fill = match level {
                Level::Inset => c.surface_alt,
                Level::Float => c.overlay,
                _ => c.surface,
            };
            let (top, bottom) = if t.dark { (c.rim, Color::WHITE.alpha(0.04)) } else { (c.rim, Color::WHITE.alpha(0.18)) };
            let mut e = e.fill(fill).border_gradient(if level == Level::Inset { 1.0 } else { 1.25 }, top, bottom);
            // A specular sheen across the top third, and a soft inner glow at the bottom edge.
            if level != Level::Inset {
                e = e.linear_gradient(std::f32::consts::PI, [
                    (0.0, Color::WHITE.alpha(if t.dark { 0.12 } else { 0.38 })),
                    (0.38, Color::WHITE.alpha(0.0)),
                    (0.85, Color::WHITE.alpha(0.0)),
                    (1.0, Color::WHITE.alpha(if t.dark { 0.05 } else { 0.14 })),
                ])
                .box_shadow(shadow(0.0, 1.0, 0.0, 0.0, Color::WHITE.alpha(if t.dark { 0.18 } else { 0.6 }), true))
                .box_shadow(shadow(0.0, if level == Level::Card { 8.0 } else { 18.0 }, if level == Level::Card { 24.0 } else { 48.0 }, -4.0, c.shadow, false));
            } else {
                e = e.box_shadow(shadow(0.0, 1.0, 3.0, 0.0, oklcha(0.2, 0.05, t.hue, if t.dark { 0.35 } else { 0.12 }), true));
            }
            if blur && t.blur > 0.0 { e.backdrop_blur(t.blur) } else { e }
        }
        Skin::Frost => {
            let fill = match level {
                Level::Inset => c.surface_alt,
                Level::Float => c.overlay,
                _ => c.surface,
            };
            let mut e = e.fill(fill).border(1.0, if level == Level::Inset { c.line } else { c.stroke });
            if level != Level::Inset {
                e = e.box_shadow(shadow(0.0, 1.0, 0.0, 0.0, c.rim, true))
                    .box_shadow(shadow(0.0, 10.0, 30.0, -6.0, c.shadow, false));
            }
            if blur && t.blur > 0.0 { e.backdrop_blur(t.blur) } else { e }
        }
        Skin::Material => {
            let (fill, y, b, a) = match level {
                Level::Inset => (c.surface_alt, 0.0, 0.0, 0.0),
                Level::Card => (c.surface, 1.0, 3.0, 0.6),
                Level::Pane => (c.surface, 0.0, 0.0, 0.0),
                Level::Float => (c.overlay, 6.0, 18.0, 1.0),
            };
            let mut e = e.fill(fill);
            if a > 0.0 {
                e = e.box_shadow(shadow(0.0, y, b, 0.0, c.shadow.alpha(c.shadow.a * a), false))
                    .box_shadow(shadow(0.0, y * 0.4, b * 0.4, 0.0, c.shadow.alpha(c.shadow.a * a * 0.8), false));
            }
            e
        }
        Skin::Bold => {
            let fill = match level {
                Level::Inset => if t.dark { c.page } else { Color::WHITE },
                Level::Float => c.overlay,
                _ => c.surface,
            };
            let off = match level {
                Level::Inset => 0.0,
                Level::Card => t.hard_shadow,
                Level::Pane => 0.0,
                Level::Float => t.hard_shadow + 2.0,
            };
            let e = e.fill(fill).border(t.stroke, c.stroke);
            if off > 0.0 { e.box_shadow(shadow(off, off, 0.0, 0.0, c.shadow, false)) } else { e }
        }
    }
}

/// A column painted as a card.
pub fn card(t: &Theme) -> El {
    paint(t, rune::col(), Level::Card, t.radius.lg, false)
}

/// A column painted as a large pane, with the backdrop blur on glass skins.
pub fn pane(t: &Theme) -> El {
    paint(t, rune::col(), Level::Pane, t.radius.lg, true)
}

/// A divider line in the theme's line colour (Bold draws it at the stroke width).
pub fn divider(t: &Theme) -> El {
    el().w_full().h(if t.skin == Skin::Bold { t.stroke } else { 1.0 }).fill(t.c.line)
}

/// The page backdrop: for glass skins, soft colour fields that give the blur something to bend;
/// for Material a tonal wash; for Bold the flat page colour. Put it at the root, under the app.
pub fn backdrop(t: &Theme) -> El {
    let h = t.hue;
    let base = el().w_full().h_full().fill(t.c.page);
    match t.skin {
        Skin::Liquid | Skin::Frost => {
            let (l, cc, a) = if t.dark { (0.55, 0.2, 0.75) } else { (0.78, 0.15, 0.9) };
            let blob = |cx: f32, cy: f32, rx: f32, ry: f32, hue: f32, alpha: f32| {
                el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .radial_gradient([cx, cy], [rx, ry], [(0.0, oklcha(l, cc, hue, alpha)), (1.0, oklcha(l, cc, hue, 0.0))])
            };
            base.child(blob(0.08, 0.12, 640.0, 540.0, h + 40.0, a))
                .child(blob(0.92, 0.16, 580.0, 500.0, h - 70.0, a))
                .child(blob(0.62, 0.98, 720.0, 540.0, h + 150.0, a))
                .child(blob(0.3, 0.72, 400.0, 400.0, h - 150.0, a * 0.8))
        }
        Skin::Material => base.linear_gradient(std::f32::consts::PI, [(0.0, oklcha(0.6, 0.08, h, if t.dark { 0.12 } else { 0.08 })), (0.5, Color::TRANSPARENT)]),
        Skin::Bold => base,
    }
}
