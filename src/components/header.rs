use leptos::*;

#[component]
pub fn Header(
    current_page: Signal<String>,
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let nav_items = vec![
        ("Home", "home"),
        ("About me", "about"),
        ("My projects", "projects"),
        ("Guestbook", "guestbook"),
        ("Contacts", "contacts"),
    ];

    let active_class = move |page: &str| {
        if current_page.get() == page {
            "nav-item active"
        } else {
            "nav-item"
        }
    };

    view! {
        <div class="header-bar">
            <div class="site-title">"★ MY RETRO WEBSITE ★"</div>
            <div class="nav-menu">
                {nav_items
                    .into_iter()
                    .map(|(label, page)| {
                        let set_page = set_current_page.clone();
                        let page = page.to_string();
                        let label = label.to_string();
                        let cls = active_class(&page);
                        view! {
                            <a
                                href="#"
                                class=cls
                                on:click=move |_| {
                                    set_page.set(page.clone());
                                }
                            >
                                {label}
                            </a>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}