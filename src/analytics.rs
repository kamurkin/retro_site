use js_sys::Reflect;
use leptos::{RwSignal, SignalSet};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Response, console};

const COUNTER_URL: &str = "https://kamurkin.goatcounter.com/counter/";
const PAGE_PATHS: [&str; 5] = ["/", "/about", "/projects", "/guestbook", "/contacts"];

pub fn load_total_count(visitor_count: RwSignal<String>) {
    spawn_local(async move {
        match fetch_total_count().await {
            Ok(count) => visitor_count.set(count),
            Err(error) => {
                console::warn_1(
                    &format!("Could not load GoatCounter total count: {error:?}").into(),
                );
            }
        }
    });
}

async fn fetch_total_count() -> Result<String, JsValue> {
    let mut total = 0u64;
    for path in PAGE_PATHS {
        total += fetch_path_count(path).await?;
    }

    Ok(format!("{total:06}"))
}

async fn fetch_path_count(path: &str) -> Result<u64, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window is unavailable"))?;
    let encoded_path = path.replace('/', "%2F");
    let url = format!("{COUNTER_URL}{encoded_path}.json");
    let response: Response = JsFuture::from(window.fetch_with_str(&url))
        .await?
        .dyn_into()?;

    if response.status() == 404 {
        return Ok(0);
    }

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "GoatCounter returned HTTP {}",
            response.status()
        )));
    }

    let json = JsFuture::from(response.json()?).await?;
    let count = Reflect::get(&json, &JsValue::from_str("count"))?
        .as_string()
        .ok_or_else(|| JsValue::from_str("GoatCounter response has no string count"))?;
    let count = count
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();
    let count = count
        .parse::<u64>()
        .map_err(|_| JsValue::from_str("GoatCounter returned an invalid count"))?;

    Ok(count)
}

pub fn track_pageview(path: &str) {
    let global = js_sys::global();
    let result = Reflect::get(&global, &JsValue::from_str("queueGoatCounter"))
        .and_then(|queue| {
            let queue = queue
                .dyn_into::<js_sys::Function>()
                .map_err(|_| JsValue::from_str("GoatCounter queue is unavailable"))?;
            queue.call1(&global, &JsValue::from_str(path)).map(|_| ())
        });

    if let Err(error) = result {
        console::warn_1(&format!("Could not track GoatCounter pageview: {error:?}").into());
    }
}
