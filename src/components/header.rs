use leptos::*;

#[component]
pub fn Header(
    current_page: Signal<String>,
    on_navigate: Callback<String>,
) -> impl IntoView {
    let nav_items = vec![
        ("Home", "home"),
        ("About me", "about"),
        ("My projects", "projects"),
        ("Guestbook", "guestbook"),
        ("Contacts", "contacts"),
    ];

    view! {
        <div class="header-bar">
            <div class="site-title">"★ MY RETRO WEBSITE ★"</div>
            <div class="nav-menu">
                {nav_items
                    .into_iter()
                    .map(|(label, page)| {
                        let on_navigate = on_navigate.clone();
                        let page = page.to_string();
                        let label = label.to_string();
                        let page_for_class = page.clone();
                        view! {
                            <a
                                href="#"
                                class=move || {
                                    if current_page.get() == page_for_class {
                                        "nav-item active"
                                    } else {
                                        "nav-item"
                                    }
                                }
                                on:click=move |ev| {
                                    ev.prevent_default();
                                    on_navigate.call(page.clone());
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