//! # glasskit
//!
//! Themable components for [Rune](https://github.com/anomalyco/rune) aimed at business apps:
//! back-office screens, chat, dashboards and task boards. Every component reads a [`Theme`],
//! and a theme is a [`Skin`] (Liquid glass, Frosted glass, Material or Bold) resolved for light
//! or dark with an accent hue. All theme fields are public, so a preset is a starting point.
//!
//! ```ignore
//! let t = Theme::with_hue(Skin::Liquid, cx.system_dark(), 275.0);
//! let save = Button::new("Salva").icon(&icons::CHECK).build(&t).on_press(|s: &mut App| s.save());
//! let root = el().w_full().h_full().child(backdrop(&t).absolute(0.0, 0.0)).child(app_ui);
//! ```
//!
//! Glass skins look their best over [`backdrop`] (or any colourful content): pass `blur: true`
//! to [`surface::paint`] (or use [`pane`]) on up to four large panes per frame for a real backdrop
//! blur; everything else draws the same tint, rim and depth without it.

pub mod chat;
pub mod controls;
pub mod data;
pub mod icons;
pub mod layout;
pub mod surface;
pub mod tasks;
pub mod theme;

pub use controls::{Button, Kind, Size, Tone, avatar, avatar_stack, badge, badge_in, checkbox, chip, icon_button, progress, segmented, status, switch, text_field};
pub use surface::{Level, backdrop, card, divider, pane, paint};
pub use theme::{Colors, Radii, Skin, Theme, Type};
