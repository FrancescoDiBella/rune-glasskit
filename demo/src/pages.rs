//! Dashboard and orders (back-office) pages.
use glasskit::data::{Column, Event, bar_chart, donut, meter, stat_card, table, timeline};
use glasskit::layout::{page_header, section};
use glasskit::{Button, Kind, Size, Theme, Tone, avatar, card, chip, divider, icons, segmented, status, text_field};
use rune::{El, col, oklch, row, text};

use crate::App;
use crate::sample::{MONTHS, ORDERS, OrderState, TREND_A, TREND_B, TREND_C, TREND_D};

fn euro(v: f32) -> String {
    let whole = v.trunc() as u64;
    let cents = ((v - v.trunc()) * 100.0).round() as u64;
    let mut s = whole.to_string();
    let mut i = s.len() as i64 - 3;
    while i > 0 {
        s.insert(i as usize, '.');
        i -= 3;
    }
    format!("€ {s},{cents:02}")
}

fn customer(t: &Theme, name: &str) -> El {
    row().gap(10.0).center_y().child(avatar(t, name, 28.0)).child(text(name.to_owned()).font(t.ty.body_strong).color(t.c.text))
}

fn order_cells(t: &Theme, i: usize) -> Vec<El> {
    let o = &ORDERS[i];
    vec![
        text(o.id).font(t.ty.mono).color(t.c.muted),
        customer(t, o.customer),
        text(o.date).font(t.ty.caption).color(t.c.muted),
        status(t, o.state.label(), o.state.tone()),
        text(euro(o.total)).font(t.ty.body_strong).color(t.c.text),
    ]
}

const ORDER_COLS: [Column; 5] = [Column::new("Ordine", 0.8), Column::new("Cliente", 2.0), Column::new("Data", 1.2), Column::new("Stato", 1.1), Column::new("Totale", 1.0).end()];

pub fn dashboard(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let stats = row().w_full().gap(16.0).flex_wrap()
        .child(stat_card(t, &icons::EURO, "Ricavi del mese", "€ 48.240", 12.4, &TREND_A, Tone::Accent))
        .child(stat_card(t, &icons::BOX, "Ordini", "1.284", 6.1, &TREND_B, Tone::Info))
        .child(stat_card(t, &icons::USERS, "Nuovi clienti", "312", -2.3, &TREND_C, Tone::Success))
        .child(stat_card(t, &icons::CHAT, "Ticket aperti", "18", -14.0, &TREND_D, Tone::Warning));
    let revenue = card(t).grow(2.0).min_w(420.0).padding(20.0).gap(18.0)
        .child(section(t, "Ricavi mensili (migliaia di €)", Some(segmented(t, &["6M", "8M", "1A"], s.range, |s: &mut App, i| s.range = i).key("range"))))
        .child(bar_chart(t, &MONTHS, Some(MONTHS.len() - 1), 170.0));
    let channels = card(t).grow(1.0).min_w(320.0).padding(20.0).gap(18.0)
        .child(section(t, "Canali di vendita", None))
        .child(donut(t, &[("Sito web", 46.0, c.accent), ("Marketplace", 28.0, c.info), ("Negozio", 18.0, c.success), ("Telefono", 8.0, c.warning)], "1.284", "ordini"));
    let recent: Vec<Vec<El>> = (0..5).map(|i| order_cells(t, i)).collect();
    let orders = col().grow(2.0).min_w(520.0).gap(12.0)
        .child(section(t, "Ordini recenti", Some(Button::new("Vedi tutti").kind(Kind::Ghost).size(Size::Small).trailing_icon(&icons::RIGHT).build(t).key("all-orders").on_press(|s: &mut App| s.go(crate::Page::Orders)))))
        .child(table(t, &ORDER_COLS, recent, &[], |_: &mut App, _| {}));
    let events = [
        Event { who: "Sara", what: "ha spedito", target: "#10480", when: "12 min fa", tone: Tone::Info },
        Event { who: "Marco", what: "ha emesso la fattura", target: "FT-2026/311", when: "40 min fa", tone: Tone::Success },
        Event { who: "Giulia", what: "ha aperto un reso su", target: "#10478", when: "1 ora fa", tone: Tone::Danger },
        Event { who: "Davide", what: "ha aggiornato lo stock di", target: "Lampada Arco", when: "3 ore fa", tone: Tone::Warning },
    ];
    let side = card(t).grow(1.0).min_w(300.0).padding(20.0).gap(16.0)
        .child(section(t, "Attività", Some(chip(t, "Live", Tone::Success))))
        .child(timeline(t, &events))
        .child(divider(t))
        .child(section(t, "Obiettivi di novembre", None))
        .child(meter(t, "Ricavi", 0.82, Tone::Accent))
        .child(meter(t, "Spedizioni in 24h", 0.94, Tone::Success))
        .child(meter(t, "Resi sotto il 3%", 0.61, Tone::Warning));
    col().w_full().gap(20.0)
        .child(page_header(t, "Buongiorno, Francesco", "Ecco come sta andando Arco Studio questo mese.", vec![
            Button::new("Esporta").kind(Kind::Secondary).icon(&icons::DOC).build(t).key("export"),
            Button::new("Nuovo ordine").icon(&icons::PLUS).build(t).key("new-order")
                .on_press(|s: &mut App| s.toast = Some("Bozza d'ordine #10483 creata".into())),
        ]))
        .child(stats)
        .child(row().w_full().gap(16.0).flex_wrap().child(revenue).child(channels))
        .child(row().w_full().gap(16.0).flex_wrap().items_start().child(orders).child(side))
}

pub const ORDER_FILTERS: [&str; 4] = ["Tutti", "Da spedire", "Spediti", "Resi"];

pub fn orders(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let q = s.search.text.to_lowercase();
    let shown: Vec<usize> = (0..ORDERS.len()).filter(|&i| {
        let o = &ORDERS[i];
        let f = match s.order_filter {
            1 => o.state == OrderState::ToShip,
            2 => o.state == OrderState::Shipped,
            3 => o.state == OrderState::Returned,
            _ => true,
        };
        f && (q.is_empty() || o.customer.to_lowercase().contains(&q) || o.id.contains(&q))
    }).collect();
    let rows: Vec<Vec<El>> = shown.iter().map(|&i| order_cells(t, i)).collect();
    let sel: Vec<usize> = shown.iter().enumerate().filter(|(_, i)| s.selected.contains(i)).map(|(k, _)| k).collect();
    let map = shown.clone();
    let tbl = table(t, &ORDER_COLS, rows, &sel, move |s: &mut App, k| {
        let i = map[k];
        if let Some(p) = s.selected.iter().position(|x| *x == i) { s.selected.remove(p); } else { s.selected.push(i); }
    });
    let total: f32 = s.selected.iter().map(|i| ORDERS[*i].total).sum();
    let bar = if s.selected.is_empty() {
        row()
    } else {
        glasskit::paint(t, row().w_full().px(16.0).py(10.0).gap(12.0).center_y(), glasskit::Level::Float, t.radius.md, false)
            .child(chip(t, &format!("{} selezionati", s.selected.len()), Tone::Accent))
            .child(text(format!("Totale {}", euro(total))).font(t.ty.body_strong).color(c.text).grow(1.0))
            .child(Button::new("Esporta CSV").kind(Kind::Outline).size(Size::Small).icon(&icons::DOC).build(t).key("csv"))
            .child(Button::new("Spedisci").size(Size::Small).icon(&icons::TRUCK).build(t).key("ship")
                .on_press(|s: &mut App| { s.toast = Some(format!("{} ordini pronti per il ritiro", s.selected.len())); s.selected.clear(); }))
    };
    let focus = s.selected.last().copied().unwrap_or(0);
    let o = &ORDERS[focus];
    let line = |l: &str, v: String| row().w_full().justify_between().child(text(l.to_owned()).font(t.ty.body).color(c.muted)).child(text(v).font(t.ty.body_strong).color(c.text));
    let detail = card(t).w(320.0).padding(20.0).gap(14.0)
        .child(row().w_full().justify_between().center_y()
            .child(text(format!("Ordine {}", o.id)).font(t.ty.heading).color(c.text))
            .child(chip(t, o.state.label(), o.state.tone())))
        .child(customer(t, o.customer))
        .child(divider(t))
        .child(line("Articoli", o.items.to_string()))
        .child(line("Subtotale", euro(o.total / 1.22)))
        .child(line("IVA 22%", euro(o.total - o.total / 1.22)))
        .child(divider(t))
        .child(line("Totale", euro(o.total)))
        .child(glasskit::controls::field(t, "Indirizzo di spedizione", text("Via dei Mille 14, 20129 Milano").font(t.ty.body).color(c.text)))
        .child(row().w_full().gap(8.0)
            .child(Button::new("Fattura").kind(Kind::Secondary).icon(&icons::DOC).build(t).key("invoice").grow(1.0).w(0.0))
            .child(Button::new("Etichetta").icon(&icons::TRUCK).build(t).key("label").grow(1.0).w(0.0)));
    col().w_full().gap(20.0)
        .child(page_header(t, "Ordini", &format!("{} ordini · {} da spedire", ORDERS.len(), ORDERS.iter().filter(|o| o.state == OrderState::ToShip).count()), vec![
            Button::new("Filtri").kind(Kind::Outline).icon(&icons::FILTER).build(t).key("filters"),
            Button::new("Nuovo ordine").icon(&icons::PLUS).build(t).key("new"),
        ]))
        .child(row().w_full().gap(12.0).center_y().flex_wrap()
            .child(segmented(t, &ORDER_FILTERS, s.order_filter, |s: &mut App, i| s.order_filter = i).key("order-filter"))
            .child(col().grow(1.0).min_w(240.0).child(text_field(t, &s.search, "Cerca cliente o numero d'ordine", Some(&icons::SEARCH), |s: &mut App| &mut s.search, |_: &mut App, _| {}).key("order-search"))))
        .child(bar)
        .child(row().w_full().gap(16.0).items_start()
            .child(col().grow(1.0).shrink().min_w(0.0).child(tbl))
            .child(detail))
}

#[allow(dead_code)]
fn tint(h: f32) -> rune::Color {
    oklch(0.7, 0.15, h)
}
