//! Sample data for the demo. Everything here is invented.
use glasskit::Tone;
use glasskit::tasks::Priority;

pub struct Order {
    pub id: &'static str,
    pub customer: &'static str,
    pub date: &'static str,
    pub items: u32,
    pub total: f32,
    pub state: OrderState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    ToShip,
    Shipped,
    Paid,
    Returned,
}

impl OrderState {
    pub fn label(self) -> &'static str {
        match self {
            OrderState::ToShip => "Da spedire",
            OrderState::Shipped => "Spedito",
            OrderState::Paid => "Pagato",
            OrderState::Returned => "Reso",
        }
    }
    pub fn tone(self) -> Tone {
        match self {
            OrderState::ToShip => Tone::Warning,
            OrderState::Shipped => Tone::Info,
            OrderState::Paid => Tone::Success,
            OrderState::Returned => Tone::Danger,
        }
    }
}

pub const ORDERS: &[Order] = &[
    Order { id: "#10482", customer: "Giulia Romano", date: "8 nov, 10:24", items: 3, total: 249.90, state: OrderState::ToShip },
    Order { id: "#10481", customer: "Marco Bianchi", date: "8 nov, 09:12", items: 1, total: 89.00, state: OrderState::Paid },
    Order { id: "#10480", customer: "Sara Ferrari", date: "7 nov, 18:40", items: 5, total: 612.40, state: OrderState::Shipped },
    Order { id: "#10479", customer: "Luca Esposito", date: "7 nov, 16:05", items: 2, total: 134.50, state: OrderState::ToShip },
    Order { id: "#10478", customer: "Chiara Colombo", date: "7 nov, 11:31", items: 1, total: 59.90, state: OrderState::Returned },
    Order { id: "#10477", customer: "Davide Ricci", date: "6 nov, 19:58", items: 4, total: 388.00, state: OrderState::Shipped },
    Order { id: "#10476", customer: "Elena Marino", date: "6 nov, 14:22", items: 2, total: 172.80, state: OrderState::Paid },
    Order { id: "#10475", customer: "Paolo Greco", date: "6 nov, 09:47", items: 6, total: 904.10, state: OrderState::Shipped },
];

pub const MONTHS: [(&str, f32); 8] = [("Apr", 31.0), ("Mag", 36.0), ("Giu", 33.0), ("Lug", 41.0), ("Ago", 29.0), ("Set", 44.0), ("Ott", 46.0), ("Nov", 48.0)];

pub const TREND_A: [f32; 12] = [12.0, 14.0, 13.0, 17.0, 16.0, 19.0, 18.0, 22.0, 21.0, 24.0, 26.0, 29.0];
pub const TREND_B: [f32; 12] = [30.0, 28.0, 31.0, 33.0, 32.0, 36.0, 35.0, 34.0, 38.0, 37.0, 40.0, 42.0];
pub const TREND_C: [f32; 12] = [22.0, 24.0, 21.0, 23.0, 20.0, 22.0, 19.0, 21.0, 18.0, 19.0, 17.0, 18.0];
pub const TREND_D: [f32; 12] = [40.0, 38.0, 41.0, 35.0, 33.0, 30.0, 31.0, 27.0, 25.0, 22.0, 20.0, 18.0];

pub struct Contact {
    pub name: &'static str,
    pub preview: &'static str,
    pub time: &'static str,
    pub unread: u32,
    pub online: bool,
}

pub const CONTACTS: &[Contact] = &[
    Contact { name: "Team Logistica", preview: "Il corriere passa alle 15:00", time: "10:42", unread: 3, online: true },
    Contact { name: "Giulia Romano", preview: "Perfetto, grazie mille!", time: "10:20", unread: 0, online: true },
    Contact { name: "Marco Bianchi", preview: "Mi mandi la fattura di ottobre?", time: "09:05", unread: 1, online: false },
    Contact { name: "Sara Ferrari", preview: "Ho caricato le nuove foto prodotto", time: "ieri", unread: 0, online: true },
    Contact { name: "Assistenza", preview: "Ticket #2231 chiuso", time: "ieri", unread: 0, online: false },
    Contact { name: "Luca Esposito", preview: "Ci sentiamo lunedì", time: "mar", unread: 0, online: false },
];

pub struct Msg {
    pub who: &'static str,
    pub body: String,
    pub time: &'static str,
    pub mine: bool,
}

pub fn messages() -> Vec<Msg> {
    let m = |who, body: &str, time, mine| Msg { who, body: body.into(), time, mine };
    vec![
        m("Sara Ferrari", "Buongiorno! Gli ordini di stamattina sono pronti per l'imballaggio.", "09:31", false),
        m("Tu", "Ottimo. Il #10482 ha tre articoli, ricordati il biglietto regalo.", "09:33", true),
        m("Davide Ricci", "Il corriere passa alle 15:00, mettiamo tutto sulla rampa 2.", "10:12", false),
        m("Tu", "Ricevuto! Preparo le etichette e le carico nel gestionale.", "10:14", true),
        m("Sara Ferrari", "Ho aggiornato anche lo stock: restano 4 pezzi della lampada Arco.", "10:40", false),
    ]
}

pub struct Task {
    pub title: &'static str,
    pub column: usize,
    pub labels: &'static [(&'static str, Tone)],
    pub priority: Priority,
    pub due: &'static str,
    pub checklist: (u32, u32),
    pub assignees: &'static [&'static str],
    pub comments: u32,
    pub done: bool,
}

pub const COLUMNS: [&str; 4] = ["Da fare", "In corso", "Revisione", "Fatto"];

pub fn tasks() -> Vec<Task> {
    vec![
        Task { title: "Nuova pagina prodotto per la collezione inverno", column: 0, labels: &[("Design", Tone::Accent)], priority: Priority::High, due: "12 nov", checklist: (1, 5), assignees: &["Sara Ferrari", "Luca Esposito"], comments: 4, done: false },
        Task { title: "Integrare il corriere con il gestionale ordini", column: 0, labels: &[("Backend", Tone::Info)], priority: Priority::Urgent, due: "9 nov", checklist: (0, 3), assignees: &["Davide Ricci"], comments: 2, done: false },
        Task { title: "Inventario di fine mese", column: 1, labels: &[("Magazzino", Tone::Warning)], priority: Priority::Medium, due: "30 nov", checklist: (6, 10), assignees: &["Giulia Romano", "Marco Bianchi", "Paolo Greco", "Elena Marino"], comments: 7, done: false },
        Task { title: "Newsletter Black Friday", column: 1, labels: &[("Marketing", Tone::Danger), ("Email", Tone::Neutral)], priority: Priority::High, due: "20 nov", checklist: (3, 4), assignees: &["Chiara Colombo"], comments: 1, done: false },
        Task { title: "Rivedere i testi delle condizioni di reso", column: 2, labels: &[("Legale", Tone::Neutral)], priority: Priority::Low, due: "15 nov", checklist: (2, 2), assignees: &["Marco Bianchi", "Elena Marino"], comments: 3, done: false },
        Task { title: "Foto dei nuovi arrivi", column: 3, labels: &[("Contenuti", Tone::Success)], priority: Priority::Medium, due: "5 nov", checklist: (8, 8), assignees: &["Sara Ferrari"], comments: 5, done: true },
    ]
}
