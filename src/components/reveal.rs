use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

/// Sets `data-revealed="1"` on the given element once it enters the viewport.
/// Each caller passes its own `NodeRef::<Tag>::get()?.unchecked_into::<Element>()` —
/// keeps this helper agnostic to which HTML tag it's observing.
pub fn observe_reveal(el: Element) {
    if let Err(e) = setup(el) {
        web_sys::console::warn_1(&format!("[reveal] {e:?}").into());
    }
}

fn setup(el: Element) -> Result<(), JsValue> {
    let cb = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
        for i in 0..entries.length() {
            let entry: web_sys::IntersectionObserverEntry =
                entries.get(i).dyn_into().unwrap();
            if entry.is_intersecting() {
                entry.target().set_attribute("data-revealed", "1").ok();
            }
        }
    });
    let init = IntersectionObserverInit::new();
    init.set_root_margin("0px 0px -10% 0px");
    init.set_threshold(&JsValue::from_f64(0.15));
    let obs = IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &init)?;
    obs.observe(&el);
    // both must outlive the page — they're page-singletons
    cb.forget();
    std::mem::forget(obs);
    Ok(())
}
