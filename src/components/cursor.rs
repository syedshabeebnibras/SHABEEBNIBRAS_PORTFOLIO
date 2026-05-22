use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

#[component]
pub fn CustomCursor() -> impl IntoView {
    let ring_ref = NodeRef::<leptos::html::Div>::new();
    let dot_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        let (Some(ring), Some(dot)) = (ring_ref.get(), dot_ref.get()) else { return };
        let ring: Element = ring.unchecked_into();
        let dot: Element = dot.unchecked_into();
        wire_cursor(ring, dot);
    });

    view! {
        <div class="cursor" aria-hidden="true">
            <div class="cursor__ring" node_ref=ring_ref></div>
            <div class="cursor__dot"  node_ref=dot_ref></div>
        </div>
    }
}

fn wire_cursor(ring: Element, dot: Element) {
    // target follows the real cursor; ring lerp-follows; dot is exact.
    let target = Rc::new(RefCell::new((0.0_f64, 0.0_f64)));
    let ring_pos = Rc::new(RefCell::new((0.0_f64, 0.0_f64)));

    // ── mousemove: update target + snap the dot
    {
        let target = target.clone();
        let dot = dot.clone();
        let cb = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
            let x = e.client_x() as f64;
            let y = e.client_y() as f64;
            *target.borrow_mut() = (x, y);
            let _ = dot.set_attribute(
                "style",
                &format!("transform: translate3d({x}px, {y}px, 0) translate(-50%, -50%)"),
            );
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .ok();
        cb.forget();
    }

    // ── mouseover: grow ring when entering a link/button/[data-magnetic]
    {
        let ring_in = ring.clone();
        let cb_over = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
            if let Some(t) = e.target() {
                if let Some(el) = t.dyn_ref::<Element>() {
                    if matches(el) {
                        ring_in.class_list().add_1("is-hot").ok();
                    }
                }
            }
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mouseover", cb_over.as_ref().unchecked_ref())
            .ok();
        cb_over.forget();
    }

    // ── mouseout: shrink ring back when leaving
    {
        let ring_out = ring.clone();
        let cb_out = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
            if let Some(t) = e.target() {
                if let Some(el) = t.dyn_ref::<Element>() {
                    if matches(el) {
                        ring_out.class_list().remove_1("is-hot").ok();
                    }
                }
            }
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mouseout", cb_out.as_ref().unchecked_ref())
            .ok();
        cb_out.forget();
    }

    // ── rAF loop: spring-follow ring toward target
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    {
        let target = target.clone();
        let ring_pos = ring_pos.clone();
        // ring's last move — into the rAF closure
        *g.borrow_mut() = Some(Closure::new(move || {
            let (tx, ty) = *target.borrow();
            let mut p = ring_pos.borrow_mut();
            p.0 += (tx - p.0) * 0.18;
            p.1 += (ty - p.1) * 0.18;
            let _ = ring.set_attribute(
                "style",
                &format!(
                    "transform: translate3d({}px, {}px, 0) translate(-50%, -50%)",
                    p.0, p.1
                ),
            );
            drop(p);
            request(f.borrow().as_ref().unwrap());
        }));
    }
    request(g.borrow().as_ref().unwrap());
}

fn matches(el: &Element) -> bool {
    let mut cur = Some(el.clone());
    while let Some(node) = cur {
        let tag = node.tag_name();
        if tag.eq_ignore_ascii_case("a") || tag.eq_ignore_ascii_case("button") {
            return true;
        }
        if node.has_attribute("data-magnetic") {
            return true;
        }
        cur = node.parent_element();
    }
    false
}

fn request(c: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(c.as_ref().unchecked_ref())
        .ok();
}
