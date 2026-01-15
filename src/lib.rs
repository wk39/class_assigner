#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MainApp;

mod class_room;

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_async<F>(fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    use once_cell::sync::OnceCell;
    use std::sync::Arc;
    use tokio::runtime::{Builder, Runtime};

    // 전역 tokio 런타임 (lazy static)
    static RUNTIME: OnceCell<Arc<Runtime>> = OnceCell::new();

    // 최초 1회 런타임 초기화
    let rt = RUNTIME.get_or_init(|| {
        Arc::new(
            Builder::new_multi_thread()
                .enable_all() // IO, timer 등 tokio 기능 활성화
                .worker_threads(2) // 필요 시 조정
                .thread_name("tokio-bg")
                .build()
                .expect("Failed to create Tokio runtime"),
        )
    });

    // 백그라운드 task 실행 (비동기 detach)
    rt.spawn(fut);
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_async<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}
