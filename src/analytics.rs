use js_sys::Reflect;
use leptos::{RwSignal, SignalSet};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Response, console};

const TOTAL_COUNT_URL: &str = "https://kamurkin.goatcounter.com/counter/TOTAL.json";

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
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Window is unavailable"))?;
    let response: Response = JsFuture::from(window.fetch_with_str(TOTAL_COUNT_URL))
        .await?
        .dyn_into()?;

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
        .parse::<u64>()
        .map_err(|_| JsValue::from_str("GoatCounter returned an invalid count"))?;

    Ok(format!("{count:06}"))
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
