use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader,
};

const VS: &str = r#"#version 300 es
in vec2 a_pos;
void main() { gl_Position = vec4(a_pos, 0.0, 1.0); }
"#;

// Fragment shader: layered FBM warp + radial bloom. Tuned to feel like
// you're looking at a slow plasma through frosted glass.
const FS: &str = r#"#version 300 es
precision highp float;
out vec4 outColor;
uniform vec2  u_res;
uniform float u_time;
uniform vec2  u_mouse;

// hash + noise (iq)
float hash(vec2 p){ p = fract(p*vec2(123.34,456.21)); p += dot(p,p+45.32); return fract(p.x*p.y); }
float noise(vec2 p){
  vec2 i = floor(p); vec2 f = fract(p);
  float a = hash(i);
  float b = hash(i+vec2(1,0));
  float c = hash(i+vec2(0,1));
  float d = hash(i+vec2(1,1));
  vec2 u = f*f*(3.0-2.0*f);
  return mix(a,b,u.x) + (c-a)*u.y*(1.0-u.x) + (d-b)*u.x*u.y;
}
float fbm(vec2 p){
  float s = 0.0; float a = 0.5;
  for(int i=0;i<5;i++){ s += a*noise(p); p *= 2.02; a *= 0.5; }
  return s;
}

void main(){
  vec2 uv = (gl_FragCoord.xy - 0.5*u_res) / u_res.y;
  vec2 m  = (u_mouse - 0.5*u_res) / u_res.y;

  float t = u_time * 0.06;
  vec2 q = uv * 1.6;
  q += 0.4 * vec2(fbm(q + t), fbm(q - t + 5.0));
  float f = fbm(q * 1.2 + t);

  // mouse-pulled warp
  float md = length(uv - m*0.25);
  f += 0.15 * exp(-md*4.0);

  // palette: deep navy → cyan → magenta highlights
  vec3 navy    = vec3(0.035, 0.041, 0.071);
  vec3 cyan    = vec3(0.000, 0.820, 0.700);
  vec3 magenta = vec3(0.900, 0.180, 0.520);

  vec3 col = mix(navy, cyan, smoothstep(0.35, 0.75, f));
  col = mix(col, magenta, smoothstep(0.78, 0.95, f));

  // soft vignette
  float v = 1.0 - smoothstep(0.6, 1.4, length(uv));
  col *= mix(0.55, 1.0, v);

  // film grain
  col += (hash(gl_FragCoord.xy + u_time) - 0.5) * 0.025;

  outColor = vec4(col, 1.0);
}
"#;

#[component]
pub fn ShaderBackground() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let canvas: HtmlCanvasElement = canvas.unchecked_into();
        if let Err(err) = init_gl(canvas) {
            web_sys::console::warn_1(&format!("[shader_bg] init failed: {err:?}").into());
        }
    });

    view! {
        <canvas
            class="shader-bg"
            node_ref=canvas_ref
            aria-hidden="true"
        />
    }
}

fn init_gl(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    let gl = canvas
        .get_context("webgl2")?
        .ok_or_else(|| JsValue::from_str("webgl2 unsupported"))?
        .dyn_into::<GL>()?;

    let program = link_program(&gl, VS, FS)?;
    gl.use_program(Some(&program));

    // fullscreen triangle
    let vbo = gl.create_buffer().ok_or("no buf")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
    let verts: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
    unsafe {
        let view = js_sys::Float32Array::view(&verts);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &view, GL::STATIC_DRAW);
    }
    let loc = gl.get_attrib_location(&program, "a_pos") as u32;
    gl.enable_vertex_attrib_array(loc);
    gl.vertex_attrib_pointer_with_i32(loc, 2, GL::FLOAT, false, 0, 0);

    let u_res = gl.get_uniform_location(&program, "u_res");
    let u_time = gl.get_uniform_location(&program, "u_time");
    let u_mouse = gl.get_uniform_location(&program, "u_mouse");

    // mouse tracking
    let mouse = std::rc::Rc::new(std::cell::Cell::new((0.0f32, 0.0f32)));
    {
        let mouse_cb = mouse.clone();
        let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            mouse_cb.set((e.client_x() as f32, e.client_y() as f32));
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // resize-aware loop
    let gl_rc = std::rc::Rc::new(gl);
    let canvas_rc = std::rc::Rc::new(canvas);
    let program_rc = std::rc::Rc::new(program);

    let f = std::rc::Rc::new(std::cell::RefCell::new(None::<Closure<dyn FnMut(f64)>>));
    let g = f.clone();

    {
        let gl_rc = gl_rc.clone();
        let canvas_rc = canvas_rc.clone();
        let mouse = mouse.clone();
        let program_rc = program_rc.clone();
        *g.borrow_mut() = Some(Closure::new(move |ts: f64| {
            let dpr = web_sys::window().unwrap().device_pixel_ratio().min(2.0) as f32;
            let win_w = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as f32;
            let win_h = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as f32;
            let want_w = (win_w * dpr) as u32;
            let want_h = (win_h * dpr) as u32;
            if canvas_rc.width() != want_w || canvas_rc.height() != want_h {
                canvas_rc.set_width(want_w);
                canvas_rc.set_height(want_h);
                gl_rc.viewport(0, 0, want_w as i32, want_h as i32);
            }

            gl_rc.use_program(Some(program_rc.as_ref()));
            if let Some(loc) = u_res.as_ref() {
                gl_rc.uniform2f(Some(loc), want_w as f32, want_h as f32);
            }
            if let Some(loc) = u_time.as_ref() {
                gl_rc.uniform1f(Some(loc), (ts * 0.001) as f32);
            }
            if let Some(loc) = u_mouse.as_ref() {
                let (mx, my) = mouse.get();
                gl_rc.uniform2f(Some(loc), mx * dpr, (win_h - my) * dpr);
            }
            gl_rc.draw_arrays(GL::TRIANGLES, 0, 3);

            request_frame(f.borrow().as_ref().unwrap());
        }));
    }
    request_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_frame(closure: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .ok();
}

fn link_program(gl: &GL, vs_src: &str, fs_src: &str) -> Result<WebGlProgram, JsValue> {
    let vs = compile_shader(gl, GL::VERTEX_SHADER, vs_src)?;
    let fs = compile_shader(gl, GL::FRAGMENT_SHADER, fs_src)?;
    let prog = gl.create_program().ok_or("create_program")?;
    gl.attach_shader(&prog, &vs);
    gl.attach_shader(&prog, &fs);
    gl.link_program(&prog);
    if !gl
        .get_program_parameter(&prog, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Err(JsValue::from_str(
            &gl.get_program_info_log(&prog).unwrap_or_default(),
        ));
    }
    Ok(prog)
}

fn compile_shader(gl: &GL, kind: u32, src: &str) -> Result<WebGlShader, JsValue> {
    let sh = gl.create_shader(kind).ok_or("create_shader")?;
    gl.shader_source(&sh, src);
    gl.compile_shader(&sh);
    if !gl
        .get_shader_parameter(&sh, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Err(JsValue::from_str(
            &gl.get_shader_info_log(&sh).unwrap_or_default(),
        ));
    }
    Ok(sh)
}
