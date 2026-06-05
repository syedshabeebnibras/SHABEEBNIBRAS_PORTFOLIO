//! Hand-rolled motion system — real per-frame physics in Rust/WASM.
//!
//! Three effects, all driven by `requestAnimationFrame` closures:
//!   * `wire_kinetic_name`  — magnetic + idle-float glyph physics on the hero
//!   * `wire_magnetic_chips`— spring-follow cursor attraction on skill chips
//!   * `start_hero_field`   — drifting warm particle field on a 2D canvas
//!
//! Everything is defensively guarded — a missing node or absent API returns
//! early instead of panicking (a WASM panic would blank the whole page).

use std::cell::{Cell, RefCell};
use std::f64::consts::TAU;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement, MouseEvent};

// ─── shared pointer tracker ──────────────────────────────────────────────
// One window-level mousemove listener feeds every physics loop. Installed
// lazily the first time `pointer()` is called.
thread_local! {
    static POINTER: RefCell<Option<Rc<Cell<(f64, f64)>>>> = const { RefCell::new(None) };
}

pub fn pointer() -> Rc<Cell<(f64, f64)>> {
    POINTER.with(|slot| {
        if let Some(existing) = slot.borrow().as_ref() {
            return existing.clone();
        }
        // start the cursor far off-screen so nothing reacts until first move
        let pos = Rc::new(Cell::new((-9999.0, -9999.0)));
        let pos_cb = pos.clone();
        let cb = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
            pos_cb.set((e.client_x() as f64, e.client_y() as f64));
        });
        if let Some(win) = web_sys::window() {
            let _ = win
                .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref());
        }
        cb.forget();
        *slot.borrow_mut() = Some(pos.clone());
        pos
    })
}

pub fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

/// True only on devices with a precise hover-capable pointer (a mouse/trackpad).
/// Phones and tablets report a coarse pointer with no hover — there's no cursor
/// to drive the magnet/tilt loops, so we skip them entirely to save battery.
pub fn pointer_is_fine() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(hover: hover) and (pointer: fine)")
                .ok()
                .flatten()
        })
        .map(|m| m.matches())
        .unwrap_or(true) // assume desktop when the query is unsupported
}

/// Combined guard used by every interactive physics loop: skip if the user
/// prefers reduced motion OR there's no fine pointer (i.e. touch device).
fn skip_physics() -> bool {
    reduced_motion() || !pointer_is_fine()
}

fn now_seconds() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now() * 0.001)
        .unwrap_or(0.0)
}

fn request_frame(c: &Closure<dyn FnMut()>) {
    if let Some(w) = web_sys::window() {
        let _ = w.request_animation_frame(c.as_ref().unchecked_ref());
    }
}

fn collect(root: &Element, selector: &str) -> Vec<HtmlElement> {
    let mut out = Vec::new();
    if let Ok(list) = root.query_selector_all(selector) {
        for i in 0..list.length() {
            if let Some(node) = list.item(i) {
                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                    out.push(el);
                }
            }
        }
    }
    out
}

// ─── 1) kinetic hero name ────────────────────────────────────────────────
// Each `.char__in` gets: a gentle index-phased idle float, plus a magnetic
// pull toward the cursor that ramps in over a radius. A `--glow` custom
// property (0..1) drives a CSS text-shadow halo near the cursor.
pub fn wire_kinetic_name(root: Element) {
    if skip_physics() {
        return;
    }
    let chars = Rc::new(collect(&root, ".char__in"));
    if chars.is_empty() {
        return;
    }
    let pointer = pointer();
    let plane_root = root.clone();
    let plane_state = Cell::new((0.0_f64, 0.0_f64));

    let radius = 150.0_f64;
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let t = now_seconds();
        let (mx, my) = pointer.get();
        // whole-name 3D plane tilt toward the cursor (depth layer)
        apply_plane_tilt(&plane_root, &plane_state, mx, my);
        for (i, el) in chars.iter().enumerate() {
            let rect = el.get_bounding_client_rect();
            let cx = rect.left() + rect.width() * 0.5;
            let cy = rect.top() + rect.height() * 0.5;
            let dx = cx - mx;
            let dy = cy - my;
            let dist = (dx * dx + dy * dy).sqrt().max(0.0001);

            // idle wave — each glyph offset in phase by its index
            let idle = (t * 1.5 + i as f64 * 0.45).sin() * 2.0;

            let (mut px, mut py, mut scale, mut glow) = (0.0, 0.0, 1.0, 0.0);
            if dist < radius {
                let force = (1.0 - dist / radius).powi(2); // smooth 0..1 ramp
                px = -(dx / dist) * force * 10.0; // pull toward cursor (gentle)
                py = -(dy / dist) * force * 10.0 - force * 7.0; // + lift
                scale = 1.0 + force * 0.18;
                glow = force;
            }

            let style = el.style();
            let _ = style.set_property(
                "transform",
                &format!(
                    "translate({:.2}px,{:.2}px) scale({:.3})",
                    px,
                    py + idle,
                    scale
                ),
            );
            let _ = style.set_property("--glow", &format!("{glow:.3}"));
        }
        request_frame(f.borrow().as_ref().unwrap());
    }));
    request_frame(g.borrow().as_ref().unwrap());
}

// ─── 1b) hero name 3D plane tilt ─────────────────────────────────────────
// Rotates the whole `.hero__name` toward the cursor like a tilting plate,
// composing with the per-glyph 2D magnetism for a layered depth effect.
fn apply_plane_tilt(root: &Element, state: &Cell<(f64, f64)>, mx: f64, my: f64) {
    let rect = root.get_bounding_client_rect();
    let cx = rect.left() + rect.width() * 0.5;
    let cy = rect.top() + rect.height() * 0.5;
    let nx = ((mx - cx) / (rect.width() * 0.5 + 1.0)).clamp(-1.4, 1.4);
    let ny = ((my - cy) / (rect.height() * 0.5 + 1.0)).clamp(-1.4, 1.4);
    let (mut prx, mut pry) = state.get();
    let trx = -ny * 3.2; // subtle — degrees
    let try_ = nx * 4.2;
    prx += (trx - prx) * 0.08;
    pry += (try_ - pry) * 0.08;
    state.set((prx, pry));
    if let Some(html) = root.dyn_ref::<HtmlElement>() {
        let _ = html.style().set_property(
            "transform",
            &format!("perspective(1400px) rotateX({prx:.2}deg) rotateY({pry:.2}deg)"),
        );
    }
}

// ─── 2) magnetic skill chips ─────────────────────────────────────────────
// Each `.chip__mag` springs toward an offset that points at the cursor when
// it's near, and relaxes back to rest otherwise. The integration `cur +=
// (target - cur) * k` is a cheap critically-damped-ish spring.
pub fn wire_magnetic_chips(root: Element) {
    if skip_physics() {
        return;
    }
    let mags = collect(&root, ".chip__mag");
    if mags.is_empty() {
        return;
    }
    // pair each node with its live spring state (current x, y)
    let chips: Rc<Vec<(HtmlElement, Cell<(f64, f64)>)>> =
        Rc::new(mags.into_iter().map(|el| (el, Cell::new((0.0, 0.0)))).collect());
    let pointer = pointer();

    let radius = 130.0_f64;
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let (mx, my) = pointer.get();
        for (el, state) in chips.iter() {
            let rect = el.get_bounding_client_rect();
            let cx = rect.left() + rect.width() * 0.5;
            let cy = rect.top() + rect.height() * 0.5;
            let dx = mx - cx;
            let dy = my - cy;
            let dist = (dx * dx + dy * dy).sqrt().max(0.0001);

            let (mut tx, mut ty) = (0.0, 0.0);
            if dist < radius {
                let force = (1.0 - dist / radius).powi(2);
                tx = (dx / dist) * force * 9.0; // drift toward cursor
                ty = (dy / dist) * force * 9.0;
            }
            let (mut x, mut y) = state.get();
            x += (tx - x) * 0.16; // spring follow
            y += (ty - y) * 0.16;
            state.set((x, y));

            let _ = el
                .style()
                .set_property("transform", &format!("translate({x:.2}px,{y:.2}px)"));
        }
        request_frame(f.borrow().as_ref().unwrap());
    }));
    request_frame(g.borrow().as_ref().unwrap());
}

// ─── 2b) 3D tilt project cards ───────────────────────────────────────────
// Each `.card__3d` rotates in 3D toward the cursor while it hovers, lifts on
// the Z axis, and exposes `--gx/--gy/--ga` for a cursor-tracking glare. All
// channels are spring-smoothed so entry/exit glides instead of snapping.
pub fn wire_tilt_cards(root: Element) {
    if skip_physics() {
        return;
    }
    let planes = collect(&root, ".card__3d");
    if planes.is_empty() {
        return;
    }
    // state per card: (rotX, rotY, lift 0..1, glare 0..1)
    let cards: Rc<Vec<(HtmlElement, Cell<(f64, f64, f64, f64)>)>> = Rc::new(
        planes
            .into_iter()
            .map(|el| (el, Cell::new((0.0, 0.0, 0.0, 0.0))))
            .collect(),
    );
    let pointer = pointer();

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let (mx, my) = pointer.get();
        for (el, st) in cards.iter() {
            let rect = el.get_bounding_client_rect();
            let cx = rect.left() + rect.width() * 0.5;
            let cy = rect.top() + rect.height() * 0.5;
            let inside = mx >= rect.left()
                && mx <= rect.right()
                && my >= rect.top()
                && my <= rect.bottom();

            let nx = ((mx - cx) / (rect.width() * 0.5 + 1.0)).clamp(-1.0, 1.0);
            let ny = ((my - cy) / (rect.height() * 0.5 + 1.0)).clamp(-1.0, 1.0);

            let (mut trx, mut try_, mut tlift, mut tgl) = (0.0, 0.0, 0.0, 0.0);
            if inside {
                trx = -ny * 6.5; // degrees
                try_ = nx * 8.5;
                tlift = 1.0;
                tgl = 1.0;
            }

            let (mut rx, mut ry, mut lift, mut gl) = st.get();
            let k = 0.12;
            rx += (trx - rx) * k;
            ry += (try_ - ry) * k;
            lift += (tlift - lift) * k;
            gl += (tgl - gl) * k;
            st.set((rx, ry, lift, gl));

            let style = el.style();
            let _ = style.set_property(
                "transform",
                &format!(
                    "rotateX({rx:.2}deg) rotateY({ry:.2}deg) translateZ({:.1}px)",
                    lift * 28.0
                ),
            );
            let gx = (nx * 0.5 + 0.5) * 100.0;
            let gy = (ny * 0.5 + 0.5) * 100.0;
            let _ = style.set_property("--gx", &format!("{gx:.1}%"));
            let _ = style.set_property("--gy", &format!("{gy:.1}%"));
            let _ = style.set_property("--ga", &format!("{:.3}", gl * 0.55));
        }
        request_frame(f.borrow().as_ref().unwrap());
    }));
    request_frame(g.borrow().as_ref().unwrap());
}

/// Convenience: wire 3D tilt across every `.card__3d` in the document.
/// Call once after the whole page has mounted (covers Work + Volunteering).
pub fn wire_tilt_all() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(body) = doc.body() {
            wire_tilt_cards(body.into());
        }
    }
}

// ─── 3) hero warm-particle field (2D canvas) ─────────────────────────────
#[derive(Clone, Copy)]
struct Particle {
    x: f64,  // normalized 0..1
    y: f64,
    vx: f64,
    vy: f64,
    r: f64,  // base radius fraction of min(w,h)
    color: u8,
}

const PALETTE: [&str; 4] = [
    "rgba(201,123,90,",  // terracotta
    "rgba(122,138,106,", // sage
    "rgba(202,161,90,",  // amber
    "rgba(176,122,122,", // dusty rose
];

pub fn start_hero_field(canvas: HtmlCanvasElement) {
    if skip_physics() {
        return;
    }
    let Ok(Some(ctx_obj)) = canvas.get_context("2d") else {
        return;
    };
    let Ok(ctx) = ctx_obj.dyn_into::<CanvasRenderingContext2d>() else {
        return;
    };

    // deterministic pseudo-random seed walk (Math.random is banned in this
    // build for resume-safety, so we derive positions from a cheap LCG).
    let mut seed = 0x2545_F491_4F6C_DD1D_u64;
    let mut rand = move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / (u32::MAX as f64)
    };

    let mut particles = Vec::with_capacity(34);
    for _ in 0..34 {
        particles.push(Particle {
            x: rand(),
            y: rand(),
            vx: (rand() - 0.5) * 0.0006,
            vy: (rand() - 0.5) * 0.0006,
            r: 0.06 + rand() * 0.09,
            color: (rand() * 4.0) as u8 % 4,
        });
    }
    let particles = Rc::new(RefCell::new(particles));
    let ctx = Rc::new(ctx);
    let canvas = Rc::new(canvas);

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.0)
            .min(2.0);
        let cw = canvas.client_width().max(1) as f64;
        let ch = canvas.client_height().max(1) as f64;
        let pw = (cw * dpr) as u32;
        let ph = (ch * dpr) as u32;
        if canvas.width() != pw || canvas.height() != ph {
            canvas.set_width(pw);
            canvas.set_height(ph);
        }
        let w = pw as f64;
        let h = ph as f64;
        let t = now_seconds();
        ctx.clear_rect(0.0, 0.0, w, h);

        let min_dim = w.min(h);
        for p in particles.borrow_mut().iter_mut() {
            // smooth flow field — no noise lib, just layered trig
            let fx = (p.x * 6.2 + t * 0.06).sin() + (p.y * 4.4 - t * 0.05).cos();
            let fy = (p.y * 5.8 - t * 0.045).cos() + (p.x * 3.9 + t * 0.04).sin();
            p.vx = (p.vx + fx * 0.000018) * 0.992 + 0.00006; // gentle rightward drift
            p.vy = (p.vy + fy * 0.000018) * 0.992;
            p.x += p.vx;
            p.y += p.vy;
            // wrap
            if p.x < -0.2 { p.x = 1.2; }
            if p.x > 1.2 { p.x = -0.2; }
            if p.y < -0.2 { p.y = 1.2; }
            if p.y > 1.2 { p.y = -0.2; }

            let px = p.x * w;
            let py = p.y * h;
            let base_r = p.r * min_dim;
            let color = PALETTE[p.color as usize];
            // 3 stacked translucent discs → soft blurred blob without filters
            for k in 0..3 {
                let rr = base_r * (1.0 + k as f64 * 0.7);
                let a = 0.05 * (1.0 - k as f64 * 0.28);
                ctx.set_global_alpha(a);
                ctx.set_fill_style_str(&format!("{color}1.0)"));
                ctx.begin_path();
                let _ = ctx.arc(px, py, rr.max(0.1), 0.0, TAU);
                ctx.fill();
            }
        }
        ctx.set_global_alpha(1.0);
        request_frame(f.borrow().as_ref().unwrap());
    }));
    request_frame(g.borrow().as_ref().unwrap());
}
