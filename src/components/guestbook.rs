use leptos::*;
use crate::app::GuestbookEntry;
use js_sys::Date;

#[component]
pub fn Guestbook(entries: RwSignal<Vec<GuestbookEntry>>) -> impl IntoView {
    let (new_author, set_new_author) = create_signal(String::new());
    let (new_message, set_new_message) = create_signal(String::new());

    let add_entry = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let author = new_author.get();
        let message = new_message.get();
        if author.is_empty() || message.is_empty() {
            return;
        }
        let date: String = Date::new_0()
            .to_locale_string("ru-RU", &js_sys::Object::new())
            .into();
        let id = entries.get().len() + 1;
        entries.update(|list| {
            list.push(GuestbookEntry {
                id,
                author,
                message,
                date,
            });
        });
        set_new_author.set(String::new());
        set_new_message.set(String::new());
    };

    let reset = move |_| {
        set_new_author.set(String::new());
        set_new_message.set(String::new());
    };

    view! {
        <div class="guestbook-section">
            <h2 class="section-title">"Guestbook"</h2>
            <div class="guestbook-intro">
                <p>"Оставьте запись в моей книге гостей!"</p>
            </div>
            <div class="guestbook-form">
                <form on:submit=add_entry>
                    <table class="form-table">
                        <tr>
                            <td class="form-label">"Имя:"</td>
                            <td>
                                <input
                                    type="text"
                                    class="form-input"
                                    prop:value=new_author
                                    on:input=move |ev| {
                                        set_new_author.set(event_target_value(&ev));
                                    }
                                />
                            </td>
                        </tr>
                        <tr>
                            <td class="form-label">"Сообщение:"</td>
                            <td>
                                <textarea
                                    class="form-textarea"
                                    prop:value=new_message
                                    on:input=move |ev| {
                                        set_new_message.set(event_target_value(&ev));
                                    }
                                ></textarea>
                            </td>
                        </tr>
                        <tr>
                            <td></td>
                            <td>
                                <button class="btn-submit" type="submit">"Добавить запись"</button>
                                <button class="btn-reset" type="reset" on:click=reset>"Очистить"</button>
                            </td>
                        </tr>
                    </table>
                </form>
            </div>
            <div class="guestbook-entries">
                <h3 class="entries-title">"Записи гостей:"</h3>
                {move || {
                    let entries_list = entries.get();
                    if entries_list.is_empty() {
                        view! {
                            <div class="no-entries">
                                <i>"Записей пока нет. Будьте первым!"</i>
                            </div>
                        }.into_view()
                    } else {
                        let mut sorted: Vec<_> = entries_list.clone();
                        sorted.sort_by(|a, b| b.id.cmp(&a.id));
                        view! {
                            <table class="entries-table">
                                <tbody>
                                    {sorted
                                        .into_iter()
                                        .map(|entry| {
                                            view! {
                                                <tr>
                                                    <td class="entry-cell">
                                                        <div class="entry-header">
                                                            <span class="entry-author">{entry.author}</span>
                                                            <span class="entry-date">{entry.date}</span>
                                                        </div>
                                                        <div class="entry-message">{entry.message}</div>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                </tbody>
                            </table>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}