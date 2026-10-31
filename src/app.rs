use leptos::*;
use crate::components::{Content, Footer, Header, Sidebar};

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = create_signal("home".to_string());
    let current_page = current_page.into();
    let guestbook_entries = create_rw_signal(Vec::<GuestbookEntry>::new());
    let visitor_count = create_rw_signal(1452u32);

    // Имитация увеличения счётчика при загрузке
    let _ = visitor_count.update(|n| *n += 1);

    view! {
        <div class="retro-container">
            <Header
                current_page=current_page
                set_current_page=set_current_page
            />
            <div class="main-layout">
                <Sidebar
                    current_page=current_page
                    set_current_page=set_current_page
                />
                <Content
                    current_page=current_page
                    guestbook_entries=guestbook_entries
                    visitor_count=visitor_count
                />
            </div>
            <Footer />
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuestbookEntry {
    pub id: usize,
    pub author: String,
    pub message: String,
    pub date: String,
}

impl Default for GuestbookEntry {
    fn default() -> Self {
        Self {
            id: 0,
            author: String::new(),
            message: String::new(),
            date: String::new(),
        }
    }
}