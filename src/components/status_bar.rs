use js_sys::{Array, Date, Intl, Object, Reflect};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[component]
pub fn StatusBar() -> impl IntoView {
    let (clock, set_clock) = signal(format_now());
    let tz_label = local_tz_city();

    // tick every second — uses local time, not UTC
    Effect::new(move |_| {
        let window = web_sys::window().expect("no window");
        let cb = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            set_clock.set(format_now());
        });
        let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            1000,
        );
        cb.forget();
    });

    view! {
        <header class="statusbar" aria-label="status">
            <div class="statusbar__cell">
                <span class="dot dot--live"></span>
                <span>"available"</span>
            </div>
            <div class="statusbar__cell statusbar__cell--mid">
                <span>"syed shabeeb nibras"</span>
            </div>
            <div class="statusbar__cell">
                <span class="muted">{format!("{tz_label} · ")}</span>
                <span class="mono">{clock}</span>
            </div>
        </header>
    }
}

/// 24h clock in the visitor's local timezone.
fn format_now() -> String {
    let d = Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        d.get_hours(),
        d.get_minutes(),
        d.get_seconds()
    )
}

/// City portion of the visitor's IANA timezone (e.g. `America/Chicago` → `chicago`).
/// Computed once at mount — timezone doesn't change while the tab is open.
fn local_tz_city() -> String {
    let fmt = Intl::DateTimeFormat::new(&Array::new(), &Object::new());
    let opts = fmt.resolved_options();
    let key = JsValue::from_str("timeZone");
    let tz = Reflect::get(&opts, &key)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();
    if tz.is_empty() {
        return "local".to_string();
    }
    tz.split('/')
        .last()
        .unwrap_or(&tz)
        .replace('_', " ")
        .to_lowercase()
}
