use leptos::*;

#[component]
pub fn Sidebar(
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
            "sidebar-item active"
        } else {
            "sidebar-item"
        }
    };

    view! {
        <div class="sidebar">
            <div class="sidebar-title">"Navigation"</div>
            <ul class="sidebar-list">
                {nav_items
                    .into_iter()
                    .map(|(label, page)| {
                        let set_page = set_current_page.clone();
                        let page = page.to_string();
                        let label = label.to_string();
                        let cls = active_class(&page);
                        view! {
                            <li>
                                <a
                                    href="#"
                                    class=cls
                                    on:click=move |_| {
                                        set_page.set(page.clone());
                                    }
                                >
                                    {label}
                                </a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </ul>
            <div class="sidebar-box">
                <div class="sidebar-box-title">"Links"</div>
                <ul class="link-list">
                    <li><a href="#" class="link">"HTML 4.01"</a></li>
                    <li><a href="#" class="link">"CSS 2.0"</a></li>
                    <li><a href="#" class="link">"Windows XP"</a></li>
                    <li><a href="#" class="link">"IE6 Best Viewed"</a></li>
                </ul>
            </div>
            <div class="sidebar-box">
                <div class="sidebar-box-title">"Site Stats"</div>
                <div class="stat-item">
                    <span class="stat-label">"Visits:"</span>
                    <span class="stat-value">"1,452"</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">"Online:"</span>
                    <span class="stat-value">"1"</span>
                </div>
                <div class="stat-item">
                    <span class="stat-label">"Since:"</span>
                    <span class="stat-value">"2003"</span>
                </div>
            </div>
            <div class="sidebar-box">
                <div class="sidebar-box-title">"Affiliates"</div>
                <div class="affiliate">
                    <a href="#" class="aff-link">"Retro Web Ring"</a>
                </div>
                <div class="affiliate">
                    <a href="#" class="aff-link">"GeoCities Mirror"</a>
                </div>
            </div>
        </div>
    }
}