//! Data display for dashboards and back-office screens: stat cards, charts, tables and feeds.
use rune::{Align, Color, El, Icon, LineCap, Shape, canvas, col, el, icon, row, text};

use crate::controls::Tone;
use crate::surface::{Level, card, divider, paint};
use crate::theme::{Skin, Theme};

/// A line sparkline with a soft area under it, `w`×`h` points.
pub fn sparkline(t: &Theme, values: &[f32], color: Color, w: f32, h: f32) -> El {
    if values.len() < 2 {
        return el().size(w, h);
    }
    let (lo, hi) = values.iter().fold((f32::MAX, f32::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
    let span = (hi - lo).max(1e-6);
    let n = values.len() as f32 - 1.0;
    let pts: Vec<[f32; 2]> = values.iter().enumerate().map(|(i, v)| [i as f32 / n * 100.0, 4.0 + (1.0 - (v - lo) / span) * 52.0]).collect();
    let mut area = pts.clone();
    area.push([100.0, 60.0]);
    area.push([0.0, 60.0]);
    let width = if t.skin == Skin::Bold { 2.75 } else { 2.0 };
    let last = *pts.last().unwrap();
    let shape = Shape::new().view_box(100.0, 60.0)
        .fill(area, color.alpha(if t.skin == Skin::Bold { 0.35 } else { 0.16 }))
        .stroke(pts, width, color)
        .stroke(vec![last, last], 6.0, color);
    canvas(shape).size(w, h)
}

/// A KPI tile: icon, label, big figure, change against the previous period and a sparkline.
pub fn stat_card(t: &Theme, i: &'static Icon, label: &str, value: &str, delta: f32, trend: &[f32], tone: Tone) -> El {
    let c = &t.c;
    let col_ = tone.color(t);
    let up = delta >= 0.0;
    let dcol = if up { c.success } else { c.danger };
    let chip = row().gap(3.0).center_y().px(7.0).py(2.0).radius(t.radius.pill.min(10.0))
        .fill(dcol.alpha(if t.skin == Skin::Bold { 1.0 } else { 0.15 }))
        .child(icon(if up { &crate::icons::UP } else { &crate::icons::DOWN }, 11.0).color(if t.skin == Skin::Bold { c.text } else { dcol }))
        .child(text(format!("{:+.1}%", delta)).font(t.ty.caption).color(if t.skin == Skin::Bold { c.text } else { dcol }));
    let chip = if t.skin == Skin::Bold { chip.border(2.0, c.stroke) } else { chip };
    let badge = row().size(34.0, 34.0).center().radius(t.radius.sm);
    let badge = match t.skin {
        Skin::Bold => badge.fill(col_).border(2.0, c.stroke).child(icon(i, 17.0).color(c.text)),
        _ => badge.fill(col_.alpha(0.16)).child(icon(i, 17.0).color(col_)),
    };
    card(t).grow(1.0).min_w(200.0).padding(18.0).gap(12.0)
        .child(row().w_full().center_y().justify_between().child(badge).child(chip))
        .child(col().gap(2.0)
            .child(text(value.to_owned()).font(t.ty.figure).color(c.text))
            .child(text(label.to_owned()).font(t.ty.caption).color(c.muted)))
        .child(sparkline(t, trend, col_, 220.0, 38.0).w_full())
}

/// Vertical bars with labels under them; `highlight` marks one bar in the accent.
pub fn bar_chart(t: &Theme, data: &[(&str, f32)], highlight: Option<usize>, height: f32) -> El {
    let c = &t.c;
    let max = data.iter().map(|d| d.1).fold(0.0, f32::max).max(1e-6);
    let bars = data.iter().enumerate().map(|(i, (label, v))| {
        let on = highlight == Some(i);
        let h = (v / max * height).max(4.0);
        let r = if t.skin == Skin::Bold { 4.0 } else { 7.0 };
        let mut bar = el().w_full().h(h).corners([r, r, 2.0, 2.0]).motion(t.spring);
        bar = match t.skin {
            Skin::Bold => bar.fill(if on { c.accent } else { c.surface_alt }).border(2.0, c.stroke),
            Skin::Liquid => bar.fill(if on { c.accent } else { c.accent.alpha(0.28) }).gradient(Color::WHITE.alpha(if on { 0.25 } else { 0.18 }), Color::WHITE.alpha(0.0)),
            _ => bar.fill(if on { c.accent } else { c.accent.alpha(0.25) }),
        };
        col().grow(1.0).gap(6.0).items_center()
            .child(text(format!("{v:.0}")).font(t.ty.caption).color(if on { c.text } else { c.faint }))
            .child(col().w_full().h(height).justify_end().child(bar))
            .child(text((*label).to_owned()).font(t.ty.caption).color(c.muted))
    });
    row().w_full().gap(if t.skin == Skin::Bold { 8.0 } else { 10.0 }).children(bars)
}

/// A donut chart of `(label, value, colour)` with the total in the middle and a legend.
pub fn donut(t: &Theme, parts: &[(&str, f32, Color)], center: &str, caption: &str) -> El {
    let c = &t.c;
    let total: f32 = parts.iter().map(|p| p.1).sum::<f32>().max(1e-6);
    let mut shape = Shape::new().view_box(100.0, 100.0);
    let width = if t.skin == Skin::Bold { 14.0 } else { 12.0 };
    let r = 38.0;
    let ring = |a0: f32, a1: f32| -> Vec<[f32; 2]> {
        let steps = ((a1 - a0) * 40.0).ceil().max(2.0) as usize;
        (0..=steps).map(|k| {
            let a = a0 + (a1 - a0) * k as f32 / steps as f32 - std::f32::consts::FRAC_PI_2;
            [50.0 + r * a.cos(), 50.0 + r * a.sin()]
        }).collect()
    };
    shape = shape.stroke_with(ring(0.0, std::f32::consts::TAU), true, width, LineCap::Butt, c.surface_alt);
    let gap = if t.skin == Skin::Bold { 0.0 } else { 0.06 };
    let mut a = 0.0;
    for (_, v, col_) in parts {
        let sweep = v / total * std::f32::consts::TAU;
        shape = shape.stroke_with(ring(a + gap / 2.0, a + sweep - gap / 2.0), false, width, if t.skin == Skin::Bold { LineCap::Butt } else { LineCap::Round }, *col_);
        a += sweep;
    }
    let chart = el().size(150.0, 150.0).child(canvas(shape).size(150.0, 150.0).absolute(0.0, 0.0))
        .child(col().size(150.0, 150.0).center().absolute(0.0, 0.0)
            .child(text(center.to_owned()).font(t.ty.title).color(c.text))
            .child(text(caption.to_owned()).font(t.ty.caption).color(c.faint)));
    let legend = col().gap(10.0).grow(1.0).children(parts.iter().map(|(l, v, col_)| {
        let sw = el().size(10.0, 10.0).radius(if t.skin == Skin::Bold { 2.0 } else { 5.0 }).fill(*col_);
        let sw = if t.skin == Skin::Bold { sw.border(2.0, c.stroke).size(12.0, 12.0) } else { sw };
        row().w_full().gap(8.0).center_y().child(sw)
            .child(text((*l).to_owned()).font(t.ty.body).color(c.text).grow(1.0))
            .child(text(format!("{:.0}%", v / total * 100.0)).font(t.ty.label).color(c.muted))
    }));
    row().w_full().gap(20.0).center_y().child(chart).child(legend)
}

/// A column definition for [`table`].
pub struct Column {
    pub title: &'static str,
    /// Flex weight; 0 means "size to content".
    pub flex: f32,
    pub align: Align,
}

impl Column {
    pub const fn new(title: &'static str, flex: f32) -> Column {
        Column { title, flex, align: Align::Start }
    }
    pub const fn end(mut self) -> Column {
        self.align = Align::End;
        self
    }
}

/// A table: a header row and `rows` of cells (one El per column). `selected` rows get the soft
/// accent; every row is keyed `row-{i}` so the app can attach handlers through `on_row`.
pub fn table<S: 'static>(t: &Theme, cols: &[Column], rows: Vec<Vec<El>>, selected: &[usize], on_row: impl Fn(&mut S, usize) + Clone + 'static) -> El {
    let c = &t.c;
    let cell = |e: El, col_: &Column| {
        let w = row().grow(col_.flex).min_w(0.0).center_y();
        let w = if col_.align == Align::End { w.justify_end() } else { w };
        w.child(e)
    };
    let mut head = row().w_full().h(40.0).px(16.0).gap(12.0).center_y();
    head = match t.skin {
        Skin::Bold => head.fill(c.surface_alt),
        Skin::Material => head,
        _ => head.fill(c.surface_alt.alpha(c.surface_alt.a * 0.6)),
    };
    for col_ in cols {
        head = head.child(cell(text(col_.title.to_uppercase()).font(t.ty.caption).color(c.faint), col_));
    }
    let mut body = col().w_full();
    for (i, cells) in rows.into_iter().enumerate() {
        let on = selected.contains(&i);
        let f = on_row.clone();
        let mut r = row().w_full().h(52.0).px(16.0).gap(12.0).center_y().key(format!("row-{i}"))
            .hover(c.hover).press(c.press).on_press(move |s: &mut S| f(s, i));
        if on {
            r = r.fill(c.accent_soft);
        }
        for (cell_el, col_) in cells.into_iter().zip(cols) {
            r = r.child(cell(cell_el, col_));
        }
        body = body.child(divider(t).opacity(if t.skin == Skin::Bold { 1.0 } else { 0.7 })).child(r);
    }
    paint(t, col(), Level::Card, t.radius.md, false).clip_rounded(t.radius.md).child(head).child(body)
}

/// One entry of an activity feed.
pub struct Event<'a> {
    pub who: &'a str,
    pub what: &'a str,
    pub target: &'a str,
    pub when: &'a str,
    pub tone: Tone,
}

/// A vertical timeline of events with a rail and dots.
pub fn timeline(t: &Theme, events: &[Event]) -> El {
    let c = &t.c;
    let n = events.len();
    col().w_full().children(events.iter().enumerate().map(|(i, e)| {
        let dot = el().size(12.0, 12.0).radius(6.0).fill(e.tone.color(t));
        let dot = match t.skin {
            Skin::Bold => dot.border(2.0, c.stroke),
            Skin::Material => dot,
            _ => dot.border(2.0, Color::WHITE.alpha(if t.dark { 0.3 } else { 0.85 })),
        };
        let rail = col().w(12.0).items_center().child(dot)
            .child(el().w(if t.skin == Skin::Bold { 2.5 } else { 1.5 }).grow(1.0).fill(if i + 1 == n { Color::TRANSPARENT } else { c.line }));
        row().w_full().gap(12.0).child(rail)
            .child(col().grow(1.0).shrink().gap(2.0).pb(16.0)
                .child(row().gap(4.0).flex_wrap()
                    .child(text(e.who.to_owned()).font(t.ty.body_strong).color(c.text))
                    .child(text(e.what.to_owned()).font(t.ty.body).color(c.muted))
                    .child(text(e.target.to_owned()).font(t.ty.body_strong).color(c.accent)))
                .child(text(e.when.to_owned()).font(t.ty.caption).color(c.faint)))
    }))
}

/// A labelled meter with the value on the right ("Storage 72%").
pub fn meter(t: &Theme, label: &str, value: f32, tone: Tone) -> El {
    col().w_full().gap(6.0)
        .child(row().w_full().justify_between()
            .child(text(label.to_owned()).font(t.ty.label).color(t.c.text))
            .child(text(format!("{:.0}%", value * 100.0)).font(t.ty.label).color(t.c.muted)))
        .child(crate::controls::progress(t, value, tone))
}
