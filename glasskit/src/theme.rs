//! Themes: a [`Skin`] (the material language) plus light/dark and an accent, resolved into plain
//! tokens. Every field is public, so an app can start from a preset and override anything.
use rune::{Color, Spring, TextStyle, oklch, oklcha};

/// The material language every component is drawn in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Skin {
    /// Apple-style Liquid Glass: clear, refractive-looking panes with a bright rim and soft depth.
    #[default]
    Liquid,
    /// Frosted glass: milky, heavily blurred panes with hairline borders.
    Frost,
    /// Material Design 3: solid tonal surfaces, no outlines, elevation by shadow.
    Material,
    /// Modern bold (neo-brutalist): solid colours, thick outlines and hard offset shadows.
    Bold,
}

impl Skin {
    pub const ALL: [Skin; 4] = [Skin::Liquid, Skin::Frost, Skin::Material, Skin::Bold];
    pub fn name(self) -> &'static str {
        match self {
            Skin::Liquid => "Liquid glass",
            Skin::Frost => "Frosted glass",
            Skin::Material => "Material",
            Skin::Bold => "Bold",
        }
    }
    /// Glass skins expect a colourful backdrop behind them.
    pub fn is_glass(self) -> bool {
        matches!(self, Skin::Liquid | Skin::Frost)
    }
}

/// Semantic colours. Components read only these.
#[derive(Clone, Debug)]
pub struct Colors {
    /// The page behind everything (glass skins paint [`crate::backdrop`] over it).
    pub page: Color,
    /// Card and panel fill.
    pub surface: Color,
    /// A second, quieter surface (wells, table headers, inputs).
    pub surface_alt: Color,
    /// Overlays: toasts, popovers, the composer.
    pub overlay: Color,
    pub text: Color,
    pub muted: Color,
    pub faint: Color,
    /// Dividers and quiet borders.
    pub line: Color,
    /// The outline colour of skins that draw one (Bold).
    pub stroke: Color,
    pub accent: Color,
    pub on_accent: Color,
    /// The accent at low strength, for selected rows and soft chips.
    pub accent_soft: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub shadow: Color,
    /// Top edge highlight of glass panes.
    pub rim: Color,
    pub hover: Color,
    pub press: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Radii {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    /// Fully round controls (pills) use this.
    pub pill: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Type {
    pub display: TextStyle,
    pub title: TextStyle,
    pub heading: TextStyle,
    pub body: TextStyle,
    pub body_strong: TextStyle,
    pub label: TextStyle,
    pub caption: TextStyle,
    pub mono: TextStyle,
    /// Big figures in stat cards.
    pub figure: TextStyle,
}

/// A resolved theme. Build one with [`Theme::new`], then adjust fields or use the `with_*` helpers.
#[derive(Clone, Debug)]
pub struct Theme {
    pub skin: Skin,
    pub dark: bool,
    pub c: Colors,
    pub radius: Radii,
    pub ty: Type,
    /// Outline width for skins that outline surfaces (Bold), 0 elsewhere.
    pub stroke: f32,
    /// Offset of Bold's hard shadow, in points.
    pub hard_shadow: f32,
    /// Backdrop blur in points for glass panes that ask for it.
    pub blur: f32,
    /// Spring for state changes (toggles, selection).
    pub spring: Spring,
    /// Hue of the accent, used to tint neutrals and tonal surfaces.
    pub hue: f32,
}

fn ts(size: f32, weight: f32) -> TextStyle {
    TextStyle::new(size, weight)
}

impl Theme {
    /// A preset for `skin` in light or dark, with the default accent hue (a blue-violet, 275).
    pub fn new(skin: Skin, dark: bool) -> Theme {
        Theme::with_hue(skin, dark, 275.0)
    }

    /// A preset whose accent and tinted neutrals follow `hue` (OKLCH degrees).
    pub fn with_hue(skin: Skin, dark: bool, hue: f32) -> Theme {
        let h = hue;
        let bold = skin == Skin::Bold;
        let w = |n: f32| if bold { (n + 150.0).min(900.0) } else { n };
        let ty = Type {
            display: ts(30.0, w(700.0)),
            title: ts(20.0, w(650.0)),
            heading: ts(15.0, w(600.0)),
            body: ts(14.0, 400.0),
            body_strong: ts(14.0, w(550.0)),
            label: ts(13.0, w(550.0)),
            caption: ts(12.0, if bold { 600.0 } else { 450.0 }),
            mono: TextStyle { family: rune::Family::Mono, ..ts(12.5, 450.0) },
            figure: ts(28.0, w(700.0)),
        };
        let accent = match (skin, dark) {
            (Skin::Bold, _) => oklch(0.72, 0.19, h),
            (_, true) => oklch(0.72, 0.16, h),
            (_, false) => oklch(0.56, 0.2, h),
        };
        let status = |l: f32, c: f32| {
            (oklch(l, c, 150.0), oklch(l + 0.08, c, 80.0), oklch(l - 0.02, c + 0.03, 25.0), oklch(l, c, 240.0))
        };
        let (success, warning, danger, info) = if dark { status(0.74, 0.15) } else { status(0.6, 0.15) };
        let white = Color::WHITE;
        let black = oklch(0.16, 0.01, h);
        let c = match (skin, dark) {
            (Skin::Liquid, false) => Colors {
                page: oklch(0.93, 0.03, h),
                surface: white.alpha(0.42),
                surface_alt: white.alpha(0.28),
                overlay: white.alpha(0.62),
                text: oklch(0.2, 0.03, h),
                muted: oklch(0.38, 0.03, h),
                faint: oklch(0.52, 0.03, h),
                line: white.alpha(0.55),
                stroke: white.alpha(0.7),
                accent,
                on_accent: white,
                accent_soft: accent.alpha(0.16),
                success, warning, danger, info,
                shadow: oklcha(0.3, 0.08, h, 0.22),
                rim: white.alpha(0.95),
                hover: white.alpha(0.22),
                press: black.alpha(0.06),
            },
            (Skin::Liquid, true) => Colors {
                page: oklch(0.18, 0.04, h),
                surface: white.alpha(0.09),
                surface_alt: white.alpha(0.06),
                overlay: oklcha(0.26, 0.03, h, 0.7),
                text: oklch(0.97, 0.01, h),
                muted: oklch(0.82, 0.02, h),
                faint: oklch(0.66, 0.02, h),
                line: white.alpha(0.14),
                stroke: white.alpha(0.3),
                accent,
                on_accent: oklch(0.16, 0.03, h),
                accent_soft: accent.alpha(0.2),
                success, warning, danger, info,
                shadow: Color::BLACK.alpha(0.4),
                rim: white.alpha(0.45),
                hover: white.alpha(0.08),
                press: white.alpha(0.12),
            },
            (Skin::Frost, false) => Colors {
                page: oklch(0.9, 0.04, h),
                surface: oklcha(0.985, 0.006, h, 0.66),
                surface_alt: oklcha(0.95, 0.012, h, 0.6),
                overlay: oklcha(0.99, 0.004, h, 0.82),
                text: oklch(0.22, 0.02, h),
                muted: oklch(0.42, 0.02, h),
                faint: oklch(0.56, 0.02, h),
                line: oklcha(0.4, 0.03, h, 0.12),
                stroke: white.alpha(0.6),
                accent,
                on_accent: white,
                accent_soft: accent.alpha(0.14),
                success, warning, danger, info,
                shadow: oklcha(0.35, 0.05, h, 0.14),
                rim: white.alpha(0.8),
                hover: oklcha(0.4, 0.03, h, 0.05),
                press: oklcha(0.4, 0.03, h, 0.09),
            },
            (Skin::Frost, true) => Colors {
                page: oklch(0.2, 0.035, h),
                surface: oklcha(0.3, 0.02, h, 0.55),
                surface_alt: oklcha(0.26, 0.02, h, 0.5),
                overlay: oklcha(0.3, 0.02, h, 0.8),
                text: oklch(0.96, 0.01, h),
                muted: oklch(0.78, 0.015, h),
                faint: oklch(0.62, 0.015, h),
                line: white.alpha(0.09),
                stroke: white.alpha(0.18),
                accent,
                on_accent: oklch(0.18, 0.03, h),
                accent_soft: accent.alpha(0.2),
                success, warning, danger, info,
                shadow: Color::BLACK.alpha(0.35),
                rim: white.alpha(0.16),
                hover: white.alpha(0.05),
                press: white.alpha(0.09),
            },
            (Skin::Material, false) => Colors {
                page: oklch(0.975, 0.012, h),
                surface: oklch(0.945, 0.022, h),
                surface_alt: oklch(0.915, 0.03, h),
                overlay: oklch(0.99, 0.008, h),
                text: oklch(0.22, 0.02, h),
                muted: oklch(0.42, 0.025, h),
                faint: oklch(0.56, 0.02, h),
                line: oklch(0.86, 0.025, h),
                stroke: oklch(0.6, 0.03, h),
                accent,
                on_accent: white,
                accent_soft: oklch(0.88, 0.07, h),
                success, warning, danger, info,
                shadow: oklcha(0.25, 0.05, h, 0.18),
                rim: Color::TRANSPARENT,
                hover: accent.alpha(0.08),
                press: accent.alpha(0.14),
            },
            (Skin::Material, true) => Colors {
                page: oklch(0.17, 0.015, h),
                surface: oklch(0.225, 0.02, h),
                surface_alt: oklch(0.27, 0.025, h),
                overlay: oklch(0.3, 0.025, h),
                text: oklch(0.94, 0.01, h),
                muted: oklch(0.78, 0.02, h),
                faint: oklch(0.62, 0.02, h),
                line: oklch(0.32, 0.02, h),
                stroke: oklch(0.55, 0.03, h),
                accent,
                on_accent: oklch(0.2, 0.05, h),
                accent_soft: oklch(0.36, 0.08, h),
                success, warning, danger, info,
                shadow: Color::BLACK.alpha(0.45),
                rim: Color::TRANSPARENT,
                hover: accent.alpha(0.1),
                press: accent.alpha(0.16),
            },
            (Skin::Bold, false) => Colors {
                page: oklch(0.95, 0.045, 95.0),
                surface: white,
                surface_alt: oklch(0.9, 0.09, h),
                overlay: white,
                text: oklch(0.14, 0.0, 0.0),
                muted: oklch(0.3, 0.0, 0.0),
                faint: oklch(0.45, 0.0, 0.0),
                line: oklch(0.14, 0.0, 0.0),
                stroke: oklch(0.14, 0.0, 0.0),
                accent,
                on_accent: oklch(0.14, 0.0, 0.0),
                accent_soft: oklch(0.88, 0.1, h),
                success: oklch(0.8, 0.18, 150.0),
                warning: oklch(0.87, 0.17, 95.0),
                danger: oklch(0.7, 0.2, 25.0),
                info: oklch(0.78, 0.13, 230.0),
                shadow: oklch(0.14, 0.0, 0.0),
                rim: Color::TRANSPARENT,
                hover: black.alpha(0.05),
                press: black.alpha(0.1),
            },
            (Skin::Bold, true) => Colors {
                page: oklch(0.2, 0.01, h),
                surface: oklch(0.27, 0.012, h),
                surface_alt: oklch(0.34, 0.06, h),
                overlay: oklch(0.3, 0.012, h),
                text: oklch(0.98, 0.0, 0.0),
                muted: oklch(0.85, 0.0, 0.0),
                faint: oklch(0.7, 0.0, 0.0),
                line: oklch(0.96, 0.0, 0.0),
                stroke: oklch(0.96, 0.0, 0.0),
                accent,
                on_accent: oklch(0.14, 0.0, 0.0),
                accent_soft: oklch(0.42, 0.1, h),
                success: oklch(0.8, 0.18, 150.0),
                warning: oklch(0.87, 0.17, 95.0),
                danger: oklch(0.72, 0.2, 25.0),
                info: oklch(0.78, 0.13, 230.0),
                shadow: oklch(0.96, 0.0, 0.0),
                rim: Color::TRANSPARENT,
                hover: white.alpha(0.06),
                press: white.alpha(0.1),
            },
        };
        let radius = match skin {
            Skin::Liquid => Radii { sm: 12.0, md: 18.0, lg: 26.0, pill: 999.0 },
            Skin::Frost => Radii { sm: 9.0, md: 14.0, lg: 20.0, pill: 999.0 },
            Skin::Material => Radii { sm: 8.0, md: 16.0, lg: 28.0, pill: 999.0 },
            Skin::Bold => Radii { sm: 6.0, md: 10.0, lg: 14.0, pill: 999.0 },
        };
        Theme {
            skin,
            dark,
            c,
            radius,
            ty,
            stroke: if bold { 2.5 } else { 0.0 },
            hard_shadow: if bold { 4.0 } else { 0.0 },
            blur: match skin {
                Skin::Liquid => 18.0,
                Skin::Frost => 34.0,
                _ => 0.0,
            },
            spring: Spring::new(0.32, 0.12),
            hue,
        }
    }

    /// Replaces the accent colour (and its soft variant), keeping everything else.
    pub fn with_accent(mut self, accent: Color) -> Theme {
        self.c.accent = accent;
        self.c.accent_soft = accent.alpha(if self.skin == Skin::Bold { 0.35 } else { 0.18 });
        self
    }

    /// Scales every corner radius (0 gives square corners).
    pub fn with_roundness(mut self, k: f32) -> Theme {
        self.radius.sm *= k;
        self.radius.md *= k;
        self.radius.lg *= k;
        if k == 0.0 {
            self.radius.pill = 0.0;
        }
        self
    }

    /// The colour to draw text in on top of `fill` (black or white, whichever reads better).
    pub fn ink_on(&self, fill: Color) -> Color {
        let [r, g, b, _] = [fill.r, fill.g, fill.b, fill.a];
        let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        if lum > 0.36 { oklch(0.16, 0.01, self.hue) } else { Color::WHITE }
    }
}
