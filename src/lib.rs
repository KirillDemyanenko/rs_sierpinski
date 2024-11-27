use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Point {
    x: f64,
    y: f64,
}

pub const WIDTH: f64 = 1000.0;
pub const HEIGHT: f64 = 700.0;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
    #[wasm_bindgen(js_namespace = Math)]
    fn floor(x: f64) -> f64;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("should have performance on window")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .expect("Canvas not exist")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("canvas canvas is not HtmlCanvasElement")
}

fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("Cannot get Canvas 2D context")
        .expect("Cannot get object")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("canvas canvas is not CanvasRenderingContext2d")
}
fn generate_color() -> String {
    let color = (random() * 2_f64.powf(24_f64)) as u64;
    format!("#{:X}", color).to_string()
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let start = performance().now();
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let context = context();
    let mut cnt: u32 = 1_000_000;
    let pts: [Point; 3] = [
        Point {
            x: WIDTH / 2.0,
            y: 0.0,
        },
        Point { x: 0.0, y: HEIGHT },
        Point {
            x: WIDTH,
            y: HEIGHT,
        },
    ];
    let mut p = Point {
        x: random() * WIDTH,
        y: random() * HEIGHT,
    };
    context.set_font("16px Roboto");
    *g.borrow_mut() = Some(Closure::new(move || {
        for _ in 0..1000 {
            if cnt == 0 {
                context.fill_text("All done!", 20.0, 20.0).expect("Cannot fill text");
                let _ = f.borrow_mut().take();
                return;
            }
            cnt -= 1;
            context.set_fill_style_str(&*generate_color());
            let num = floor(random() * 3.0) as usize;
            p.x = (p.x + pts[num].x) / 2.0;
            p.y = (p.y + pts[num].y) / 2.0;
            context.fill_rect(p.x, p.y, 1.0, 1.0);
            let duration = (performance().now() - start).round() as u64 / 1000;
            context.set_fill_style_str("#000");
            let step_text = ["Steps left", &*cnt.to_string()].join(" ");
            let time_text = ["Elapsed time:", &*duration.to_string(), "seconds"].join(" ");
            context.clear_rect(0.0, 700.0, WIDTH, HEIGHT);
            context.fill_text(&*step_text, 20.0, 750.0).expect("Cannot fill text");
            context.fill_text(&*time_text, 150.0, 750.0).expect("Cannot fill text");
        }
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
