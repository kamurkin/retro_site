use leptos::*;
use crate::components::{Content, Footer, Header, Sidebar};
use crate::analytics::{load_total_count, track_pageview};

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = create_signal("home".to_string());
    let visitor_count = create_rw_signal("------".to_string());
    load_total_count(visitor_count);
    track_pageview("/");

    let on_navigate = Callback::new({
        let current_page = current_page;
        let set_current_page = set_current_page;
        move |page: String| {
            if current_page.get_untracked() != page {
                set_current_page.set(page.clone());
                let path = if page == "home" {
                    "/".to_string()
                } else {
                    format!("/{page}")
                };
                track_pageview(&path);
            }
        }
    });
    let current_page = current_page.into();
    let guestbook_entries = create_rw_signal(Vec::<GuestbookEntry>::new());

    view! {
        <div class="retro-container">
            <Header
                current_page=current_page
                on_navigate=on_navigate
            />
            <div class="main-layout">
                <Sidebar
                    current_page=current_page
                    on_navigate=on_navigate
                    visitor_count=visitor_count
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