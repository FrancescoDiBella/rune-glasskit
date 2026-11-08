//! glasskit demo: one back-office app (dashboard, chat, task board, orders, component gallery)
//! that switches live between the four skins, light and dark, and any accent hue.
use glasskit::layout::{brand, nav_item, sidebar};
use glasskit::{Skin, Theme, avatar, backdrop, icon_button, icons, segmented};
use rune::{AppSpec, Cx, El, ScrollSpec, TextEdit, Tx, col, el, row, text};

mod pages;
mod pages2;
mod sample;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Orders,
    Chat,
    Tasks,
    Gallery,
}

const PAGES: [(Page, &str, &rune::Icon); 5] = [
    (Page::Dashboard, "Dashboard", &icons::GRID),
    (Page::Orders, "Ordini", &icons::BOX),
    (Page::Chat, "Messaggi", &icons::CHAT),
    (Page::Tasks, "Attività", &icons::BOARD),
    (Page::Gallery, "Componenti", &icons::SPARK),
];

pub const HUES: [f32; 6] = [275.0, 230.0, 160.0, 60.0, 25.0, 340.0];

pub struct App {
    pub skin: Skin,
    pub dark: Option<bool>,
    pub hue: f32,
    pub page: Page,
    // orders
    pub order_filter: usize,
    pub selected: Vec<usize>,
    pub search: TextEdit,
    // chat
    pub contact: usize,
    pub messages: Vec<sample::Msg>,
    pub draft: TextEdit,
    // tasks
    pub tasks: Vec<sample::Task>,
    pub task_view: usize,
    pub toast: Option<String>,
    // gallery
    pub switches: [bool; 3],
    pub checks: [bool; 3],
    pub seg: usize,
    pub field: TextEdit,
    pub chat_search: TextEdit,
    pub range: usize,
}

fn init() -> App {
    let args = rune::extra_args();
    let has = |s: &str| args.iter().any(|a| a == s);
    let skin = if has("frost") { Skin::Frost } else if has("material") { Skin::Material } else if has("bold") { Skin::Bold } else { Skin::Liquid };
    let page = if has("orders") { Page::Orders } else if has("chat") { Page::Chat } else if has("tasks") { Page::Tasks } else if has("gallery") { Page::Gallery } else { Page::Dashboard };
    let hue = args.iter().find_map(|a| a.strip_prefix("hue=").and_then(|h| h.parse().ok())).unwrap_or(275.0);
    App {
        skin,
        dark: if has("dark") { Some(true) } else if has("light") { Some(false) } else { None },
        hue,
        page,
        order_filter: 0,
        selected: vec![0, 3],
        search: TextEdit::new(""),
        contact: 0,
        messages: sample::messages(),
        draft: TextEdit::new(""),
        tasks: sample::tasks(),
        task_view: if has("list") { 1 } else { 0 },
        toast: None,
        switches: [true, false, true],
        checks: [true, false, true],
        seg: 1,
        field: TextEdit::new("Arco Studio S.r.l."),
        chat_search: TextEdit::new(""),
        range: 1,
    }
}

impl App {
    pub fn go(&mut self, p: Page) {
        self.page = p;
        self.toast = None;
    }
}

fn top_bar(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let skins: Vec<&str> = Skin::ALL.iter().map(|k| k.name()).collect();
    let pick = segmented(t, &skins, Skin::ALL.iter().position(|k| *k == s.skin).unwrap_or(0), |s: &mut App, i| s.skin = Skin::ALL[i]).key("skins");
    let swatches = row().gap(6.0).center_y().children(HUES.iter().map(|h| {
        let on = (s.hue - h).abs() < 0.5;
        let hh = *h;
        let sw = el().size(22.0, 22.0).radius(if s.skin == Skin::Bold { 5.0 } else { 11.0 })
            .fill(rune::oklch(if t.dark || s.skin == Skin::Bold { 0.72 } else { 0.56 }, 0.18, hh))
            .role("radio").checked(on).label(format!("Accento {hh}")).focusable(true).press_scale(0.9)
            .on_press(move |s: &mut App| s.hue = hh).key(format!("hue-{hh}"));
        let sw = if s.skin == Skin::Bold { sw.border(if on { 3.0 } else { 2.0 }, c.stroke) } else { sw };
        if on { sw.outline(2.0, 2.0, c.text) } else { sw }
    }));
    let dark = t.dark;
    let mode = icon_button(t, if dark { &icons::SUN } else { &icons::MOON }, "Tema", false).key("mode")
        .on_press(move |s: &mut App| s.dark = Some(!dark));
    let bar = row().w_full().h(64.0).px(16.0).gap(16.0).center_y()
        .child(col().gap(1.0).grow(1.0).shrink()
            .child(text(PAGES.iter().find(|p| p.0 == s.page).map_or("", |p| p.1)).font(t.ty.title).color(c.text))
            .child(text("Arco Studio · gestionale").font(t.ty.caption).color(c.faint)))
        .child(pick)
        .child(swatches)
        .child(mode)
        .child(icon_button(t, &icons::BELL, "Notifiche", false).key("bell"))
        .child(avatar(t, "Francesco Di Bella", 34.0));
    match t.skin {
        Skin::Bold => bar.fill(c.surface).border_sides([0.0, 0.0, t.stroke, 0.0], c.stroke),
        Skin::Material => bar,
        _ => glasskit::paint(t, bar, glasskit::Level::Pane, t.radius.lg, true),
    }
}

fn view(s: &App, cx: &mut Cx<App>) -> El {
    let dark = s.dark.unwrap_or_else(|| cx.system_dark());
    let t = Theme::with_hue(s.skin, dark, s.hue);
    let items = PAGES.iter().map(|(p, label, i)| {
        let p = *p;
        let count = match p {
            Page::Chat => Some(4),
            Page::Orders => Some(sample::ORDERS.iter().filter(|o| o.state == sample::OrderState::ToShip).count() as u32),
            _ => None,
        };
        nav_item(&t, i, label, s.page == p, count).key(format!("nav-{label}")).on_press(move |s: &mut App| s.go(p))
    }).collect();
    let footer = col().w_full().gap(10.0)
        .child(glasskit::card(&t).padding(14.0).gap(8.0)
            .child(text("Spazio di archiviazione").font(t.ty.label).color(t.c.text))
            .child(glasskit::progress(&t, 0.72, glasskit::Tone::Neutral))
            .child(text("72 GB di 100 GB").font(t.ty.caption).color(t.c.faint)))
        .child(nav_item(&t, &icons::SETTINGS, "Impostazioni", false, None).key("nav-settings"));
    let side = sidebar(&t, brand(&t, &icons::SPARK, "Arco Studio", "glasskit demo"), items, footer);
    let content = match s.page {
        Page::Dashboard => pages::dashboard(s, &t),
        Page::Orders => pages::orders(s, &t),
        Page::Chat => pages2::chat(s, &t),
        Page::Tasks => pages2::tasks(s, &t),
        Page::Gallery => pages2::gallery(s, &t),
    };
    let gap = if t.skin == Skin::Bold || t.skin == Skin::Material { 0.0 } else { 14.0 };
    let main = col().grow(1.0).shrink().min_w(0.0).h_full().gap(gap)
        .child(top_bar(s, &t))
        .child(col().key(format!("scroll-{:?}", s.page as u8)).w_full().grow(1.0).shrink().min_h(0.0)
            .scroll(ScrollSpec { stick_bottom: false })
            .child(col().w_full().padding(if gap > 0.0 { 6.0 } else { 24.0 }).pb(32.0).gap(20.0).child(content)));
    let shell = row().w_full().h_full().padding(gap).gap(gap).child(side).child(main);
    let mut root = el().w_full().h_full().fill(t.c.page)
        .child(backdrop(&t).pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(shell.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)));
    if let Some(msg) = &s.toast {
        root = root.child(col().pin(None, None, Some(28.0), Some(28.0))
            .child(glasskit::layout::toast(&t, &icons::CHECK, "Fatto", msg, glasskit::Tone::Success).key("toast")));
    }
    root
}

pub fn send(s: &mut App, tx: &mut Tx) {
    let body = s.draft.text.trim().to_owned();
    if body.is_empty() {
        return;
    }
    s.messages.push(sample::Msg { who: "Tu", body, time: "ora", mine: true });
    s.draft.clear();
    tx.scroll_to_bottom("thread");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn start() {
    rune::run(AppSpec::new("glasskit", (1360.0, 860.0), init, view));
}
