#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Class Assigner",
        native_options,
        Box::new(|cc| Ok(Box::new(class_assigner::MainApp::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        // prevent browser refresh
        prevent_close();

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(class_assigner::MainApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn prevent_close() {
    use eframe::wasm_bindgen::JsCast as _;
    use eframe::wasm_bindgen::prelude::*;
    {
        let window = web_sys::window().unwrap();

        // "beforeunload" 이벤트 핸들러 설정
        let closure = Closure::wrap(Box::new(move |event: web_sys::BeforeUnloadEvent| {
            // 이 설정을 하면 브라우저가 표준 경고창을 띄웁니다.
            event.set_return_value("작업 중인 내용이 있습니다.");
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("beforeunload", closure.as_ref().unchecked_ref())
            .unwrap();

        // 메모리 누수 방지를 위해 핸들러를 영구 유지 (앱 종료 전까지)
        closure.forget();
    }
}
