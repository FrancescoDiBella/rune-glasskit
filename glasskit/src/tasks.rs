//! Task management: kanban cards and columns, and checklist rows for to-do lists.
use rune::{Color, El, col, el, icon, row, text};

use crate::controls::{Tone, avatar_stack, checkbox, chip, progress};
use crate::icons;
use crate::surface::{Level, paint};
use crate::theme::{Skin, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn tone(self) -> Tone {
        match self {
            Priority::Low => Tone::Info,
            Priority::Medium => Tone::Success,
            Priority::High => Tone::Warning,
            Priority::Urgent => Tone::Danger,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Priority::Low => "Bassa",
            Priority::Medium => "Media",
            Priority::High => "Alta",
            Priority::Urgent => "Urgente",
        }
    }
}

/// What a kanban card shows.
pub struct TaskCard<'a> {
    pub title: &'a str,
    pub labels: &'a [(&'a str, Tone)],
    pub priority: Priority,
    pub due: &'a str,
    /// Done and total checklist items.
    pub checklist: (u32, u32),
    pub assignees: &'a [&'a str],
    pub comments: u32,
    pub selected: bool,
}

pub fn task_card(t: &Theme, k: &TaskCard) -> El {
    let c = &t.c;
    let pc = k.priority.tone().color(t);
    let mut top = row().grow(1.0).shrink().min_w(0.0).gap(6.0).flex_wrap().center_y();
    for (l, tone) in k.labels {
        top = top.child(chip(t, l, *tone));
    }
    let flag = row().gap(4.0).center_y().child(icon(&icons::FLAG, 13.0).color(pc)).child(text(k.priority.label()).font(t.ty.caption).color(pc));
    let (done, total) = k.checklist;
    let mut body = col().w_full().gap(10.0)
        .child(row().w_full().gap(8.0).center_y().child(top).child(flag))
        .child(text(k.title.to_owned()).font(t.ty.body_strong).color(c.text).wrap());
    if total > 0 {
        body = body.child(row().w_full().gap(8.0).center_y()
            .child(col().grow(1.0).child(progress(t, done as f32 / total as f32, if done == total { Tone::Success } else { Tone::Neutral })))
            .child(text(format!("{done}/{total}")).font(t.ty.caption).color(c.muted)));
    }
    body = body.child(row().w_full().center_y().justify_between()
        .child(row().gap(12.0).center_y()
            .child(row().gap(4.0).center_y().child(icon(&icons::CALENDAR, 13.0).color(c.faint)).child(text(k.due.to_owned()).font(t.ty.caption).color(c.muted)))
            .child(row().gap(4.0).center_y().child(icon(&icons::CHAT, 13.0).color(c.faint)).child(text(k.comments.to_string()).font(t.ty.caption).color(c.muted))))
        .child(avatar_stack(t, k.assignees, 24.0, 3)));
    let mut card = row().w_full().clip_rounded(t.radius.md);
    card = paint(t, card, Level::Card, t.radius.md, false).hover(c.hover).press_scale(0.985).motion(t.spring);
    if k.selected {
        card = card.outline(if t.skin == Skin::Bold { 3.0 } else { 2.0 }, 2.0, c.accent);
    }
    // A priority stripe down the leading edge (a solid block in Bold).
    let stripe = el().w(if t.skin == Skin::Bold { 8.0 } else { 4.0 }).h_full().fill(pc);
    let stripe = if t.skin == Skin::Bold { stripe.border_sides([0.0, t.stroke, 0.0, 0.0], c.stroke) } else { stripe };
    card.child(stripe).child(body.padding(14.0).grow(1.0).shrink())
}

/// A kanban column: header with a coloured marker, count and an add button, then its cards.
pub fn board_column(t: &Theme, title: &str, marker: Color, cards: Vec<El>, footer: El) -> El {
    let c = &t.c;
    let n = cards.len();
    let dot = el().size(10.0, 10.0).radius(if t.skin == Skin::Bold { 2.0 } else { 5.0 }).fill(marker);
    let dot = if t.skin == Skin::Bold { dot.border(2.0, c.stroke).size(12.0, 12.0) } else { dot };
    let count = text(n.to_string()).font(t.ty.caption).color(c.muted).px(7.0).py(1.0).radius(9.0).fill(c.surface_alt);
    let head = row().w_full().h(32.0).gap(8.0).center_y()
        .child(dot)
        .child(text(title.to_owned()).font(t.ty.heading).color(c.text))
        .child(count)
        .child(el().grow(1.0))
        .child(icon(&icons::MORE, 16.0).color(c.faint));
    let mut colm = col().w(290.0).shrink().gap(10.0).padding(10.0);
    colm = match t.skin {
        Skin::Bold => colm.radius(t.radius.lg).fill(c.surface_alt).border(t.stroke, c.stroke),
        Skin::Material => colm.radius(t.radius.lg).fill(c.surface),
        _ => paint(t, colm, Level::Inset, t.radius.lg, false),
    };
    colm.child(head).children(cards).child(footer)
}

/// A to-do row: checkbox, title (struck through when done), due date and priority.
pub fn task_row(t: &Theme, title: &str, done: bool, due: &str, priority: Priority, who: &str) -> El {
    let c = &t.c;
    let pc = priority.tone().color(t);
    let title_el = text(title.to_owned()).font(t.ty.body).color(if done { c.faint } else { c.text });
    let title_el = if done { title_el.strike(0, title.len(), None) } else { title_el };
    row().w_full().h(48.0).px(12.0).gap(12.0).center_y().radius(t.radius.sm).hover(c.hover)
        .child(checkbox(t, done).key(format!("check-{title}")))
        .child(title_el.grow(1.0).shrink())
        .child(chip(t, priority.label(), priority.tone()).opacity(if done { 0.5 } else { 1.0 }))
        .child(row().w(92.0).gap(4.0).center_y().justify_end()
            .child(icon(&icons::CLOCK, 13.0).color(if done { c.faint } else { pc }))
            .child(text(due.to_owned()).font(t.ty.caption).color(c.muted)))
        .child(crate::controls::avatar(t, who, 26.0))
}
