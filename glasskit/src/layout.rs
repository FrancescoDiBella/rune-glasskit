//! App structure: sidebar navigation, top bar, section headings, tabs, toasts and empty states.
use rune::{Color, El, Icon, Shadow, col, el, icon, row, text};

use crate::controls::{Tone, badge, solid};
use crate::surface::{Level, paint};
use crate::theme::{Skin, Theme};

/// One navigation entry. Attach `.on_press` to navigate.
pub fn nav_item(t: &Theme, i: &'static Icon, label: &str, active: bool, count: Option<u32>) -> El {
    let c = &t.c;
    let r = if t.skin == Skin::Material { t.radius.pill } else { t.radius.sm + 2.0 };
    let mut e = row().w_full().h(40.0).px(12.0).gap(12.0).center_y().role("link").label(label.to_owned()).focusable(true).motion(t.spring);
    let ink = if active {
        match t.skin {
            Skin::Bold => c.on_accent,
            Skin::Material => c.text,
            _ => c.text,
        }
    } else {
        c.muted
    };
    e = if active {
        match t.skin {
            Skin::Bold => solid(t, e, c.accent, r),
            Skin::Material => e.radius(r).fill(c.accent_soft),
            _ => paint(t, e, Level::Float, r, false),
        }
    } else {
        e.radius(r).hover(c.hover).press(c.press)
    };
    e = e.child(icon(i, 18.0).color(if active && t.skin != Skin::Bold { c.accent } else { ink }))
        .child(text(label.to_owned()).font(if active { t.ty.body_strong } else { t.ty.body }).color(ink).grow(1.0));
    if let Some(n) = count {
        e = e.child(badge(t, n));
    }
    e
}

/// The brand block at the top of a sidebar: a mark and a name.
pub fn brand(t: &Theme, mark: &'static Icon, name: &str, tagline: &str) -> El {
    let c = &t.c;
    let logo = solid(t, row().size(36.0, 36.0).center(), c.accent, if t.skin == Skin::Bold { t.radius.sm } else { 11.0 })
        .child(icon(mark, 19.0).color(if t.skin == Skin::Bold { c.on_accent } else { t.ink_on(c.accent) }));
    row().w_full().gap(10.0).center_y().px(6.0).py(4.0).child(logo)
        .child(col().gap(1.0).child(text(name.to_owned()).font(t.ty.heading).color(c.text))
            .child(text(tagline.to_owned()).font(t.ty.caption).color(c.faint)))
}

/// A sidebar pane holding `items` with `footer` pinned at the bottom.
pub fn sidebar(t: &Theme, top: El, items: Vec<El>, footer: El) -> El {
    let c = &t.c;
    let label = |s: &str| text(s.to_uppercase()).font(t.ty.caption).color(c.faint).px(12.0).pt(14.0).pb(6.0);
    let mut list = col().w_full().gap(3.0).child(label("Menu"));
    for i in items {
        list = list.child(i);
    }
    let p = col().w(248.0).h_full().padding(14.0).gap(8.0);
    let p = match t.skin {
        Skin::Bold => p.fill(c.surface).border_sides([0.0, t.stroke, 0.0, 0.0], c.stroke),
        Skin::Material => p.fill(c.surface),
        _ => paint(t, p, Level::Pane, t.radius.lg, true),
    };
    p.child(top).child(list).child(el().grow(1.0)).child(footer)
}

/// A page heading: title, subtitle and trailing actions.
pub fn page_header(t: &Theme, title: &str, subtitle: &str, actions: Vec<El>) -> El {
    let c = &t.c;
    let mut acts = row().gap(8.0).center_y();
    for a in actions {
        acts = acts.child(a);
    }
    row().w_full().center_y().justify_between().gap(16.0).flex_wrap()
        .child(col().gap(4.0).shrink()
            .child(text(title.to_owned()).font(t.ty.display).color(c.text))
            .child(text(subtitle.to_owned()).font(t.ty.body).color(c.muted)))
        .child(acts)
}

/// A section title with optional trailing content ("Ordini recenti … Vedi tutti").
pub fn section(t: &Theme, title: &str, trailing: Option<El>) -> El {
    let mut r = row().w_full().center_y().justify_between().child(text(title.to_owned()).font(t.ty.heading).color(t.c.text));
    if let Some(e) = trailing {
        r = r.child(e);
    }
    r
}

/// Underlined tabs. `on_pick(i)` receives the chosen tab.
pub fn tabs<S: 'static>(t: &Theme, labels: &[&str], selected: usize, on_pick: impl Fn(&mut S, usize) + Clone + 'static) -> El {
    let c = &t.c;
    let mut r = row().gap(4.0).role("tablist");
    for (i, l) in labels.iter().enumerate() {
        let on = i == selected;
        let f = on_pick.clone();
        let bar = el().w_full().h(if t.skin == Skin::Bold { 4.0 } else { 2.5 }).radius(2.0).fill(if on { c.accent } else { Color::TRANSPARENT }).motion(t.spring);
        let bar = if on && t.skin == Skin::Bold { bar.border(1.5, c.stroke) } else { bar };
        r = r.child(col().gap(8.0).pt(6.0).px(4.0).role("tab").checked(on).focusable(true).label((*l).to_owned())
            .on_press(move |s: &mut S| f(s, i))
            .child(text((*l).to_owned()).font(if on { t.ty.body_strong } else { t.ty.body }).color(if on { c.text } else { c.muted }).px(6.0))
            .child(bar));
    }
    r
}

/// A toast: icon, message and a tone stripe.
pub fn toast(t: &Theme, i: &'static Icon, title: &str, body: &str, tone: Tone) -> El {
    let c = &t.c;
    let col_ = tone.color(t);
    let mark = row().size(30.0, 30.0).radius(15.0).center().fill(col_.alpha(if t.skin == Skin::Bold { 1.0 } else { 0.18 }))
        .child(icon(i, 16.0).color(if t.skin == Skin::Bold { c.text } else { col_ }));
    let mark = if t.skin == Skin::Bold { mark.border(2.0, c.stroke) } else { mark };
    paint(t, row().w(340.0).padding(12.0).gap(12.0).center_y(), Level::Float, t.radius.md, false)
        .child(mark)
        .child(col().grow(1.0).shrink().gap(2.0)
            .child(text(title.to_owned()).font(t.ty.body_strong).color(c.text))
            .child(text(body.to_owned()).font(t.ty.caption).color(c.muted).wrap()))
}

/// An empty state: a big icon, a line of explanation and an action.
pub fn empty_state(t: &Theme, i: &'static Icon, title: &str, body: &str, action: El) -> El {
    let c = &t.c;
    let well = row().size(64.0, 64.0).center().radius(if t.skin == Skin::Bold { t.radius.md } else { 32.0 });
    let well = match t.skin {
        Skin::Bold => well.fill(c.accent).border(t.stroke, c.stroke).box_shadow(Shadow { x: 4.0, y: 4.0, blur: 0.0, spread: 0.0, color: c.shadow, inset: false }),
        Skin::Material => well.fill(c.accent_soft),
        _ => paint(t, well, Level::Float, 32.0, false),
    };
    col().w_full().padding(32.0).gap(12.0).items_center()
        .child(well.child(icon(i, 28.0).color(if t.skin == Skin::Bold { c.on_accent } else { c.accent })))
        .child(text(title.to_owned()).font(t.ty.title).color(c.text))
        .child(text(body.to_owned()).font(t.ty.body).color(c.muted).max_w(360.0).wrap())
        .child(action)
}
