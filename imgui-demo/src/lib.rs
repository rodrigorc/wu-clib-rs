extern crate wu_clib;

use std::rc::Rc;
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::*;

use easy_imgui_renderer::*;
use easy_imgui as imgui;
use easy_imgui_sys::*;

pub struct Data {
    render: Renderer,
    app: App,
    last_time: f32,
}

#[wasm_bindgen]
pub unsafe fn init_demo() -> *mut Data {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);
    let gl = Rc::new(gl);
    let render = Renderer::new(gl.clone()).unwrap();
    let app = App {
        _gl: gl.clone(),
    };

    let data = Box::new(Data {
        render,
        app,
        last_time: 0.0,
    });
    Box::into_raw(data)
}

#[wasm_bindgen]
pub unsafe fn do_frame(data: *mut Data, time: f32, w: i32, h: i32) {
    let data = &mut *data;
    data.render.set_size([w as f32, h as f32].into(), 1.0);
    let io = &mut *ImGui_GetIO();
    io.DeltaTime = (time - data.last_time) / 1000.0;
    data.last_time = time;
    data.render.do_frame(&mut data.app);
}
#[wasm_bindgen]
pub unsafe fn do_mouse_move(_data: *mut Data, x: i32, y: i32) {
    let io = &mut *ImGui_GetIO();
    ImGuiIO_AddMousePosEvent(io, x as f32, y as f32);
}
#[wasm_bindgen]
pub unsafe fn do_mouse_button(_data: *mut Data, btn: i32, down: bool) {
    let io = &mut *ImGui_GetIO();
    ImGuiIO_AddMouseButtonEvent(io, btn, down);
}
#[wasm_bindgen]
pub unsafe fn do_mouse_wheel(_data: *mut Data, x: i32, y: i32) {
    let io = &mut *ImGui_GetIO();
    ImGuiIO_AddMouseWheelEvent(io, x as f32, y as f32);
}

struct App {
    _gl: glr::GlContext,
}

impl imgui::UiBuilder for App {
    fn do_ui(&mut self, ui: &imgui::Ui<Self>) {
        ui.show_demo_window(None);
    }
}


