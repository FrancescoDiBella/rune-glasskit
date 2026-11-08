//! Chat: conversation rows, message bubbles, day separators, typing indicator and composer.
use rune::{Color, El, Shadow, TextEdit, Tx, col, el, icon, row, text, text_input};

use crate::controls::{avatar, badge_in, icon_button, solid};
use crate::icons;
use crate::surface::{Level, paint};
use crate::theme::{Skin, Theme};

/// A row in the conversation list.
pub struct Conversation<'a> {
    pub name: &'a str,
    pub preview: &'a str,
    pub time: &'a str,
    pub unread: u32,
    pub online: bool,
    pub selected: bool,
}

pub fn conversation(t: &Theme, cv: &Conversation) -> El {
    let c = &t.c;
    let mut av = el().size(42.0, 42.0).child(avatar(t, cv.name, 42.0).absolute(0.0, 0.0));
    if cv.online {
        let dot = el().size(12.0, 12.0).radius(6.0).fill(c.success).absolute(31.0, 30.0)
            .border(if t.skin == Skin::Bold { 2.0 } else { 2.0 }, if t.skin == Skin::Bold { c.stroke } else { c.page });
        av = av.child(dot);
    }
    let mut meta = col().gap(6.0).items_end().child(text(cv.time.to_owned()).font(t.ty.caption).color(if cv.unread > 0 { c.accent } else { c.faint }));
    if cv.unread > 0 {
        meta = meta.child(badge_in(t, cv.unread, c.accent, if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) }));
    }
    let mut r = row().w_full().px(12.0).py(10.0).gap(12.0).center_y().radius(t.radius.md).hover(c.hover).press(c.press)
        .child(av)
        .child(col().grow(1.0).shrink().min_w(0.0).gap(3.0)
            .child(text(cv.name.to_owned()).font(t.ty.body_strong).color(c.text))
            .child(text(cv.preview.to_owned()).font(t.ty.caption).color(c.muted).max_lines(1)))
        .child(meta);
    if cv.selected {
        r = match t.skin {
            Skin::Bold => r.fill(c.accent_soft).border(t.stroke, c.stroke),
            Skin::Material => r.fill(c.accent_soft),
            _ => paint(t, r, Level::Float, t.radius.md, false),
        };
    }
    r
}

/// Delivery state shown under one's own messages.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    Sent,
    Read,
}

/// A message bubble. `mine` aligns it right in the accent.
pub fn bubble(t: &Theme, body: &str, time: &str, mine: bool, delivery: Option<Delivery>) -> El {
    let c = &t.c;
    let r = t.radius.lg.min(20.0);
    let corners = if mine { [r, r, 4.0, r] } else { [r, r, r, 4.0] };
    let ink = if mine { if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) } } else { c.text };
    let mut b = col().max_w(440.0).px(14.0).py(9.0).gap(4.0);
    b = if mine {
        match t.skin {
            Skin::Liquid => b.fill(c.accent)
                .linear_gradient(std::f32::consts::PI, [(0.0, Color::WHITE.alpha(0.25)), (0.6, Color::WHITE.alpha(0.0))])
                .border_gradient(1.0, Color::WHITE.alpha(0.6), Color::WHITE.alpha(0.05))
                .box_shadow(Shadow { x: 0.0, y: 6.0, blur: 16.0, spread: -4.0, color: c.accent.alpha(0.5), inset: false }),
            Skin::Bold => b.fill(c.accent).border(t.stroke, c.stroke).box_shadow(Shadow { x: 3.0, y: 3.0, blur: 0.0, spread: 0.0, color: c.shadow, inset: false }),
            _ => b.fill(c.accent),
        }
    } else {
        match t.skin {
            Skin::Bold => b.fill(c.surface).border(t.stroke, c.stroke).box_shadow(Shadow { x: 3.0, y: 3.0, blur: 0.0, spread: 0.0, color: c.shadow, inset: false }),
            Skin::Material => b.fill(c.surface_alt),
            _ => paint(t, b, Level::Card, r, false),
        }
    };
    let b = b.corners(corners);
    let mut foot = row().gap(4.0).center_y().justify_end().child(text(time.to_owned()).font(t.ty.caption).color(ink.alpha(0.7)));
    if let Some(d) = delivery {
        let tick = icon(&icons::CHECK, 12.0).color(if d == Delivery::Read { if mine { ink } else { c.info } } else { ink.alpha(0.6) });
        foot = foot.child(tick);
    }
    let b = b.child(text(body.to_owned()).font(t.ty.body).color(ink).wrap()).child(foot);
    let line = row().w_full();
    if mine { line.justify_end().child(b) } else { line.child(b) }
}

/// A message from someone else with their avatar beside it.
pub fn bubble_from(t: &Theme, who: &str, body: &str, time: &str) -> El {
    row().w_full().gap(8.0).items_end().child(avatar(t, who, 28.0)).child(col().grow(1.0).shrink().child(bubble(t, body, time, false, None)))
}

/// A centred day label between messages.
pub fn day_separator(t: &Theme, label: &str) -> El {
    let c = &t.c;
    let pill = text(label.to_owned()).font(t.ty.caption).color(c.muted).px(10.0).py(4.0);
    let pill = match t.skin {
        Skin::Bold => pill.radius(t.radius.sm).fill(c.surface).border(2.0, c.stroke),
        Skin::Material => pill.radius(t.radius.pill).fill(c.surface_alt),
        _ => paint(t, pill, Level::Float, t.radius.pill, false),
    };
    row().w_full().center().child(pill)
}

/// Three dots that say someone is typing.
pub fn typing(t: &Theme, who: &str) -> El {
    let c = &t.c;
    let dots = row().gap(4.0).px(12.0).py(10.0)
        .children((0..3).map(|i| el().size(7.0, 7.0).radius(3.5).fill(c.muted.alpha(0.45 + 0.2 * i as f32))));
    let dots = match t.skin {
        Skin::Bold => dots.radius(14.0).fill(c.surface).border(2.0, c.stroke),
        Skin::Material => dots.radius(14.0).fill(c.surface_alt),
        _ => paint(t, dots, Level::Card, 14.0, false),
    };
    row().gap(8.0).center_y().child(avatar(t, who, 24.0)).child(dots)
        .child(text(format!("{who} sta scrivendo…")).font(t.ty.caption).color(c.faint))
}

/// The message composer: attach, a growing text field and send.
pub fn composer<S: 'static>(
    t: &Theme,
    edit: &TextEdit,
    placeholder: &str,
    access: impl for<'a> Fn(&'a mut S) -> &'a mut TextEdit + 'static,
    send: impl Fn(&mut S, &mut Tx) + Clone + 'static,
) -> El {
    let c = &t.c;
    let r = if matches!(t.skin, Skin::Liquid | Skin::Material) { 26.0 } else { t.radius.md };
    let s2 = send.clone();
    let input = text_input(edit, t.ty.body).placeholder(placeholder.to_owned(), c.faint).color(c.text)
        .grow(1.0).min_h(24.0).py(4.0).on_edit(access).on_submit(move |s: &mut S, tx| s2(s, tx));
    let send_btn = solid(t, row().size(40.0, 40.0).center().role("button").label("Invia").focusable(true).press_scale(0.92), c.accent, if t.skin == Skin::Bold { t.radius.sm } else { 20.0 })
        .child(icon(&icons::SEND, 17.0).color(if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) }))
        .on_press_tx(move |s: &mut S, tx| send(s, tx)).key("send");
    paint(t, row().w_full().padding(6.0).gap(6.0).center_y(), Level::Float, r, false)
        .child(icon_button(t, &icons::CLIP, "Allega", false).key("attach"))
        .child(input)
        .child(send_btn)
}
