use leptos::*;
use crate::{app::GuestbookEntry, components::Guestbook};

#[component]
pub fn Content(
    current_page: Signal<String>,
    guestbook_entries: RwSignal<Vec<GuestbookEntry>>,
    visitor_count: RwSignal<u32>,
) -> impl IntoView {
    view! {
        <div class="content">
            <div class="content-box">
                {move || match current_page.get().as_str() {
                    "home" => view! {
                        <HomePage visitor_count=visitor_count />
                    }.into_view(),
                    "about" => view! {
                        <AboutPage />
                    }.into_view(),
                    "projects" => view! {
                        <ProjectsPage />
                    }.into_view(),
                    "guestbook" => view! {
                        <Guestbook entries=guestbook_entries />
                    }.into_view(),
                    "contacts" => view! {
                        <ContactsPage />
                    }.into_view(),
                    _ => view! {
                        <HomePage visitor_count=visitor_count />
                    }.into_view(),
                }}
            </div>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-section">
            <h2 class="section-title">"About Me"</h2>
            <div class="about-text">
                <p>"Привет! Я автор этого сайта. Мне интересны компьютеры, программирование и старые технологии. Сейчас я изучаю Rust и создаю собственные проекты."</p>
                <p>"Мои интересы:"</p>
                <ul class="about-list">
                    <li>"Программирование и веб-разработка"</li>
                    <li>"Компьютеры, комплектующие и операционные системы"</li>
                    <li>"Ретро-технологии и старый Windows"</li>
                    <li>"Видеоигры: Dota 2, osu! и Geometry Dash"</li>
                    <li>"Музыка и цифровая культура"</li>
                </ul>
                <p>"Мне нравится разбираться в том, как всё устроено, экспериментировать с технологиями и создавать что-нибудь собственными руками."</p>
                <p>"Спасибо, что заглянули на мою страничку! Не забудьте оставить запись в гостевой книге :D"</p>
            </div>
        </div>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <div class="projects-section">
            <h2 class="section-title">"My Projects"</h2>
            <table class="projects-table">
                <thead>
                    <tr>
                        <th>Проект</th>
                        <th>Описание</th>
                        <th>Год</th>
                        <th>Статус</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Retro WebSite"</td>
                        <td>"Личный сайт в стиле интернета начала 2000-х"</td>
                        <td>"2026"</td>
                        <td>"Active"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn ContactsPage() -> impl IntoView {
    view! {
        <div class="contacts-section">
            <h2 class="section-title">"Contacts"</h2>
            <div class="contacts-info">
                <p><strong>"Email:"</strong> "Пока не указан"</p>
                <p><strong>"Discord:"</strong> "Пока не указан"</p>
                <p><strong>"Другие контакты:"</strong> "Возможно, появятся позже."</p>
            </div>
            <div class="contact-note">
                <em>"Спасибо за интерес к моему сайту! Буду рад вашим сообщениям в гостевой книге."</em>
            </div>
        </div>
    }
}

#[component]
fn HomePage(visitor_count: RwSignal<u32>) -> impl IntoView {
    view! {
        <div class="welcome-section">
            <h1 class="welcome-title">"Welcome to my website!"</h1>
            <div class="welcome-text">
                <p>"Добро пожаловать на мой персональный сайт! Этот сайт создан в стиле интернета начала 2000-х годов."</p>
                <p>"Здесь вы найдёте информацию обо мне, мои проекты, книгу гостей и контакты. Сайт оптимизирован для просмотра в разрешении 1366×768 и лучшим образом отображается в Internet Explorer 6."</p>
                <br />
                <p>"Последнее обновление: 31 октября 2026 года"</p>
            </div>
        </div>
        <div class="updates-section">
            <h2 class="section-title">"Последние обновления"</h2>
            <table class="updates-table">
                <thead>
                    <tr>
                        <th>Дата</th>
                        <th>Запись</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"31.10.2026"</td>
                        <td>"Сайт запущен! Добавлена главная страница."</td>
                    </tr>
                    <tr>
                        <td>"30.10.2026"</td>
                        <td>"Начата разработка дизайна в стиле 2000-х."</td>
                    </tr>
                    <tr>
                        <td>"29.10.2026"</td>
                        <td>"Зарегистрирован домен и выбран хостинг."</td>
                    </tr>
                    <tr>
                        <td>"28.10.2026"</td>
                        <td>"Идея создания ретро-сайта родилась."</td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="counter-section">
            <div class="counter-box">
                <div class="counter-label">"Счётчик посещений:"</div>
                <div class="counter-value">
                    {move || format!("{:06}", visitor_count.get())}
                </div>
            </div>
        </div>
    }
}