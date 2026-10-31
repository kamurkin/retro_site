use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer-bar">
            <div class="footer-content">
                <div class="footer-left">
                    <span>"© 2003–2026 My Retro Website. All rights reserved."</span>
                </div>
                <div class="footer-center">
                    <span>"Best viewed in 1024×768"</span>
                </div>
                <div class="footer-right">
                    <span>"Powered by Leptos + WASM"</span>
                </div>
            </div>
            <div class="footer-buttons">
                <button class="btn-retro">"Home"</button>
                <button class="btn-retro">"About"</button>
                <button class="btn-retro">"Contact"</button>
            </div>
        </div>
    }
}