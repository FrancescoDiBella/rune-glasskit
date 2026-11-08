//! A small 16-point line icon set (stroke and fill use `currentColor`, tinted by the component).
use rune::Icon;

macro_rules! icons {
    ($($id:ident $name:literal $body:literal;)*) => {
        $(pub static $id: Icon = Icon { name: concat!("glasskit-", $name), view: 16.0, body: $body };)*
    };
}

icons! {
    HOME "home" r#"<path d="M2.5 7.5L8 3L13.5 7.5V13H9.75V9.75H6.25V13H2.5Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    GRID "grid" r#"<rect x="2.5" y="2.5" width="4.5" height="4.5" rx="1.2" stroke="currentColor" stroke-width="1.3" fill="none"/><rect x="9" y="2.5" width="4.5" height="4.5" rx="1.2" stroke="currentColor" stroke-width="1.3" fill="none"/><rect x="2.5" y="9" width="4.5" height="4.5" rx="1.2" stroke="currentColor" stroke-width="1.3" fill="none"/><rect x="9" y="9" width="4.5" height="4.5" rx="1.2" stroke="currentColor" stroke-width="1.3" fill="none"/>"#;
    CHAT "chat" r#"<path d="M2.5 4A1.5 1.5 0 0 1 4 2.5H12A1.5 1.5 0 0 1 13.5 4V9.5A1.5 1.5 0 0 1 12 11H7L4 13.5V11H4A1.5 1.5 0 0 1 2.5 9.5Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    BOARD "board" r#"<rect x="2.5" y="2.5" width="11" height="11" rx="2" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M6.25 2.5V13.5M9.75 2.5V13.5" stroke="currentColor" stroke-width="1.3"/><rect x="3.5" y="4" width="1.8" height="3" rx=".6" fill="currentColor"/><rect x="7.1" y="4" width="1.8" height="5" rx=".6" fill="currentColor"/>"#;
    BOX "box" r#"<path d="M2.5 5L8 2.5L13.5 5V11L8 13.5L2.5 11Z M2.5 5L8 7.5L13.5 5 M8 7.5V13.5" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    USERS "users" r#"<circle cx="6" cy="5.5" r="2.25" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M2 13C2.4 10.6 4 9.5 6 9.5S9.6 10.6 10 13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M10.5 3.5A2 2 0 0 1 10.5 7.5M11.5 9.6C12.8 10 13.7 11.1 14 13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>"#;
    CHART "chart" r#"<path d="M2.5 13.5H13.5M4.5 11V8M8 11V4.5M11.5 11V6.5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>"#;
    CALENDAR "calendar" r#"<rect x="2.5" y="3.5" width="11" height="10" rx="2" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M2.5 6.5H13.5M5.5 2V4.5M10.5 2V4.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>"#;
    CLOCK "clock" r#"<circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M8 5V8L10 9.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>"#;
    FLAG "flag" r#"<path d="M3.5 13.5V2.5M3.5 3H11.5L10 5.75L11.5 8.5H3.5" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" stroke-linecap="round" fill="none"/>"#;
    SEARCH "search" r#"<circle cx="7" cy="7" r="4" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M10 10L13 13" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>"#;
    PLUS "plus" r#"<path d="M8 3.5V12.5M3.5 8H12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>"#;
    CHECK "check" r#"<path d="M3.5 8.5L6.5 11.5L12.5 4.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#;
    CLOSE "close" r#"<path d="M4.5 4.5L11.5 11.5M11.5 4.5L4.5 11.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>"#;
    SEND "send" r#"<path d="M2.5 8L13.5 2.5L10.5 13.5L8 9Z M8 9L13.5 2.5" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    CLIP "clip" r#"<path d="M11.5 7.5L7.25 11.75A2.5 2.5 0 0 1 3.7 8.2L8.6 3.3A1.75 1.75 0 0 1 11.1 5.8L6.5 10.4A.9.9 0 0 1 5.2 9.1L9 5.3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>"#;
    MORE "more" r#"<circle cx="3.5" cy="8" r="1.3" fill="currentColor"/><circle cx="8" cy="8" r="1.3" fill="currentColor"/><circle cx="12.5" cy="8" r="1.3" fill="currentColor"/>"#;
    BELL "bell" r#"<path d="M4 11V7.25A4 4 0 0 1 12 7.25V11L13 12.25H3Z M6.75 13.75H9.25" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" stroke-linecap="round" fill="none"/>"#;
    SETTINGS "settings" r#"<circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M8 1.75V3.5M8 12.5V14.25M1.75 8H3.5M12.5 8H14.25M3.6 3.6L4.8 4.8M11.2 11.2L12.4 12.4M3.6 12.4L4.8 11.2M11.2 4.8L12.4 3.6" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>"#;
    UP "up" r#"<path d="M8 12.5V3.5M4.5 7L8 3.5L11.5 7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#;
    DOWN "down" r#"<path d="M8 3.5V12.5M4.5 9L8 12.5L11.5 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#;
    RIGHT "right" r#"<path d="M6 3.5L10.5 8L6 12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#;
    LEFT "left" r#"<path d="M10 3.5L5.5 8L10 12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#;
    FILTER "filter" r#"<path d="M2.5 3.5H13.5L9.5 8.5V13L6.5 11.5V8.5Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    SPARK "spark" r#"<path d="M8 2L9.4 6.6L14 8L9.4 9.4L8 14L6.6 9.4L2 8L6.6 6.6Z" fill="currentColor"/>"#;
    SUN "sun" r#"<circle cx="8" cy="8" r="2.75" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M8 1.5V3M8 13V14.5M1.5 8H3M13 8H14.5M3.4 3.4L4.5 4.5M11.5 11.5L12.6 12.6M3.4 12.6L4.5 11.5M11.5 4.5L12.6 3.4" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>"#;
    MOON "moon" r#"<path d="M12.75 9.75A5.25 5.25 0 0 1 6.25 3.25A5.25 5.25 0 1 0 12.75 9.75Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/>"#;
    DOC "doc" r#"<path d="M4 2H9.5L12.5 5V14H4Z M9.5 2V5H12.5 M6 8.5H10.5M6 11H9" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" stroke-linecap="round" fill="none"/>"#;
    EURO "euro" r#"<path d="M11.5 4.25A4.25 4.25 0 1 0 11.5 11.75M3 7H9M3 9H8.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>"#;
    TRUCK "truck" r#"<path d="M1.5 4H9.5V11H1.5Z M9.5 6.5H12.25L14.5 9V11H9.5" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/><circle cx="4.5" cy="11.75" r="1.25" fill="currentColor"/><circle cx="11.75" cy="11.75" r="1.25" fill="currentColor"/>"#;
}
