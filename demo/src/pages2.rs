//! Chat, task board and component gallery pages.
use glasskit::chat::{Conversation, Delivery, bubble, bubble_from, composer, conversation, day_separator, typing};
use glasskit::layout::{empty_state, page_header, section, tabs, toast};
use glasskit::tasks::{TaskCard, board_column, task_card, task_row};
use glasskit::{Button, Kind, Size, Theme, Tone, avatar, avatar_stack, badge, card, checkbox, chip, divider, icon_button, icons, progress, segmented, status, switch, text_field};
use rune::{El, ScrollSpec, col, row, text};

use crate::App;
use crate::sample::{COLUMNS, CONTACTS};

pub fn chat(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let list = CONTACTS.iter().enumerate().fold(col().w_full().gap(2.0), |l, (i, k)| {
        l.child(conversation(t, &Conversation { name: k.name, preview: k.preview, time: k.time, unread: if i == s.contact { 0 } else { k.unread }, online: k.online, selected: i == s.contact })
            .key(format!("contact-{i}")).on_press(move |s: &mut App| s.contact = i))
    });
    let left = card(t).w(320.0).h(680.0).padding(12.0).gap(12.0)
        .child(row().w_full().justify_between().center_y().px(4.0)
            .child(text("Messaggi").font(t.ty.title).color(c.text))
            .child(icon_button(t, &icons::PLUS, "Nuova chat", true).key("new-chat")))
        .child(text_field(t, &s.chat_search, "Cerca nelle chat", Some(&icons::SEARCH), |s: &mut App| &mut s.chat_search, |_: &mut App, _| {}).key("chat-search"))
        .child(list);
    let k = &CONTACTS[s.contact];
    let header = row().w_full().px(18.0).py(14.0).gap(12.0).center_y()
        .child(avatar(t, k.name, 40.0))
        .child(col().grow(1.0).gap(2.0)
            .child(text(k.name).font(t.ty.heading).color(c.text))
            .child(status(t, if k.online { "Online · 5 membri" } else { "Ultimo accesso ieri" }, if k.online { Tone::Success } else { Tone::Neutral })))
        .child(avatar_stack(t, &["Sara Ferrari", "Davide Ricci", "Giulia Romano", "Marco Bianchi", "Paolo Greco"], 28.0, 3))
        .child(icon_button(t, &icons::SEARCH, "Cerca", false))
        .child(icon_button(t, &icons::MORE, "Altro", false));
    let mut thread = col().w_full().gap(10.0).padding(18.0).child(day_separator(t, "Oggi"));
    for m in &s.messages {
        thread = thread.child(if m.mine {
            bubble(t, &m.body, m.time, true, Some(Delivery::Read))
        } else {
            bubble_from(t, m.who, &m.body, m.time)
        });
    }
    thread = thread.child(typing(t, "Davide Ricci"));
    let right = card(t).grow(1.0).shrink().min_w(0.0).h(680.0).clip_rounded(t.radius.lg)
        .child(header)
        .child(divider(t))
        .child(col().key("thread").w_full().grow(1.0).shrink().min_h(0.0).scroll(ScrollSpec { stick_bottom: true }).child(thread))
        .child(col().w_full().padding(14.0).child(composer(t, &s.draft, "Scrivi un messaggio…", |s: &mut App| &mut s.draft, crate::send).key("composer")));
    row().w_full().gap(16.0).child(left).child(right)
}

pub fn tasks(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let markers = [c.info, c.warning, c.accent, c.success];
    let view_tabs = tabs(t, &["Bacheca", "Lista"], s.task_view, |s: &mut App, i| s.task_view = i).key("task-tabs");
    let head = page_header(t, "Attività del team", "Sprint 46 · 6 attività · clicca una scheda per spostarla avanti", vec![
        avatar_stack(t, &["Sara Ferrari", "Davide Ricci", "Giulia Romano", "Marco Bianchi", "Paolo Greco", "Elena Marino"], 32.0, 4),
        Button::new("Nuova attività").icon(&icons::PLUS).build(t).key("new-task"),
    ]);
    let body = if s.task_view == 0 {
        let mut board = row().gap(16.0).items_start();
        for (ci, name) in COLUMNS.iter().enumerate() {
            let cards: Vec<El> = s.tasks.iter().enumerate().filter(|(_, k)| k.column == ci).map(|(i, k)| {
                task_card(t, &TaskCard { title: k.title, labels: k.labels, priority: k.priority, due: k.due, checklist: k.checklist, assignees: k.assignees, comments: k.comments, selected: false })
                    .key(format!("task-{i}"))
                    .on_press(move |s: &mut App| {
                        let next = (s.tasks[i].column + 1) % COLUMNS.len();
                        s.tasks[i].column = next;
                        s.tasks[i].done = next == COLUMNS.len() - 1;
                        s.toast = Some(format!("Spostata in \u{201c}{}\u{201d}", COLUMNS[next]));
                    })
            }).collect();
            let add = Button::new("Aggiungi").kind(Kind::Ghost).size(Size::Small).icon(&icons::PLUS).full_width().build(t).key(format!("add-{ci}"));
            board = board.child(board_column(t, name, markers[ci], cards, add));
        }
        col().key("board-scroll").w_full().child(board)
    } else {
        let mut list = card(t).w_full().padding(8.0).gap(2.0);
        for (i, k) in s.tasks.iter().enumerate() {
            if i > 0 {
                list = list.child(divider(t).opacity(0.6));
            }
            list = list.child(task_row(t, k.title, k.done, k.due, k.priority, k.assignees[0]).key(format!("row-{i}"))
                .on_press(move |s: &mut App| { s.tasks[i].done = !s.tasks[i].done; s.tasks[i].column = if s.tasks[i].done { 3 } else { 1 }; }));
        }
        list
    };
    col().w_full().gap(18.0).child(head).child(view_tabs).child(body)
}

fn demo_card(t: &Theme, title: &str, body: El) -> El {
    card(t).grow(1.0).min_w(300.0).padding(20.0).gap(16.0).child(section(t, title, None)).child(body)
}

pub fn gallery(s: &App, t: &Theme) -> El {
    let c = &t.c;
    let buttons = col().gap(12.0)
        .child(row().gap(10.0).flex_wrap()
            .child(Button::new("Primario").build(t))
            .child(Button::new("Secondario").kind(Kind::Secondary).build(t))
            .child(Button::new("Contorno").kind(Kind::Outline).build(t))
            .child(Button::new("Ghost").kind(Kind::Ghost).build(t))
            .child(Button::new("Elimina").kind(Kind::Danger).build(t)))
        .child(row().gap(10.0).flex_wrap().center_y()
            .child(Button::new("Piccolo").size(Size::Small).build(t))
            .child(Button::new("Con icona").icon(&icons::SEND).build(t))
            .child(Button::new("Grande").size(Size::Large).trailing_icon(&icons::RIGHT).build(t))
            .child(icon_button(t, &icons::BELL, "Notifiche", false))
            .child(icon_button(t, &icons::PLUS, "Aggiungi", true)));
    let labels = col().gap(14.0)
        .child(row().gap(8.0).flex_wrap().child(chip(t, "Neutro", Tone::Neutral)).child(chip(t, "Accento", Tone::Accent)).child(chip(t, "Successo", Tone::Success))
            .child(chip(t, "Attenzione", Tone::Warning)).child(chip(t, "Errore", Tone::Danger)).child(chip(t, "Info", Tone::Info)))
        .child(row().gap(16.0).flex_wrap().center_y().child(status(t, "Pagato", Tone::Success)).child(status(t, "In attesa", Tone::Warning)).child(status(t, "Scaduto", Tone::Danger))
            .child(badge(t, 3)).child(badge(t, 128)))
        .child(row().gap(16.0).center_y().child(avatar(t, "Giulia Romano", 40.0)).child(avatar(t, "Marco Bianchi", 32.0))
            .child(avatar_stack(t, &["Sara Ferrari", "Davide Ricci", "Elena Marino", "Paolo Greco", "Luca Esposito"], 32.0, 3)));
    let toggle = |i: usize, label: &str, on: bool| {
        row().w_full().justify_between().center_y().child(text(label.to_owned()).font(t.ty.body).color(c.text))
            .child(switch(t, on).key(format!("sw-{i}")).on_press(move |s: &mut App| s.switches[i] = !s.switches[i]))
    };
    let check = |i: usize, label: &str, on: bool| {
        row().gap(10.0).center_y().child(checkbox(t, on).key(format!("cb-{i}")).on_press(move |s: &mut App| s.checks[i] = !s.checks[i]))
            .child(text(label.to_owned()).font(t.ty.body).color(c.text))
    };
    let controls = col().gap(14.0)
        .child(toggle(0, "Notifiche push", s.switches[0]))
        .child(toggle(1, "Modalità ferie", s.switches[1]))
        .child(toggle(2, "Fatturazione automatica", s.switches[2]))
        .child(divider(t))
        .child(row().gap(18.0).flex_wrap().child(check(0, "IVA inclusa", s.checks[0])).child(check(1, "Spedizione express", s.checks[1])).child(check(2, "Regalo", s.checks[2])))
        .child(segmented(t, &["Giorno", "Settimana", "Mese"], s.seg, |s: &mut App, i| s.seg = i).key("seg"));
    let inputs = col().gap(14.0)
        .child(glasskit::controls::field(t, "Ragione sociale", text_field(t, &s.field, "Nome azienda", None, |s: &mut App| &mut s.field, |_: &mut App, _| {}).key("company")))
        .child(text_field(t, &s.search, "Cerca prodotti, clienti, ordini…", Some(&icons::SEARCH), |s: &mut App| &mut s.search, |_: &mut App, _| {}).key("g-search"))
        .child(col().gap(8.0).child(text("Avanzamento").font(t.ty.label).color(c.text)).child(progress(t, 0.35, Tone::Neutral)).child(progress(t, 0.7, Tone::Success)).child(progress(t, 0.9, Tone::Danger)));
    let feedback = col().gap(14.0)
        .child(toast(t, &icons::CHECK, "Fattura inviata", "FT-2026/311 inviata a marco.bianchi@example.com", Tone::Success))
        .child(toast(t, &icons::BELL, "Stock basso", "Restano 4 pezzi della lampada Arco", Tone::Warning));
    let empty = card(t).grow(1.0).min_w(300.0).child(empty_state(t, &icons::BOX, "Nessun reso", "Quando un cliente apre un reso, lo trovi qui con lo stato del rimborso.",
        Button::new("Crea reso manuale").kind(Kind::Secondary).icon(&icons::PLUS).build(t)));
    col().w_full().gap(18.0)
        .child(page_header(t, "Componenti", &format!("Tutti i componenti di glasskit nella skin {}", t.skin.name()), vec![]))
        .child(row().w_full().gap(16.0).flex_wrap().items_start()
            .child(demo_card(t, "Pulsanti", buttons))
            .child(demo_card(t, "Etichette e avatar", labels)))
        .child(row().w_full().gap(16.0).flex_wrap().items_start()
            .child(demo_card(t, "Controlli", controls))
            .child(demo_card(t, "Campi e progresso", inputs)))
        .child(row().w_full().gap(16.0).flex_wrap().items_start()
            .child(demo_card(t, "Notifiche", feedback))
            .child(empty))
}
