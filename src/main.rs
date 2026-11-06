//! iOS・デスクトップの実行体。iOS は egui-proxy の wgpu（Metal）入口へ、デスクトップは glow 入口へ渡す。
//! 起動画面は環境変数 `FANDHE_PROXY_APP`（未指定ならメニュー）で選ぶ。Android は何もしない
//! （入口は `lib.rs` の `android_main`）。

#[cfg(target_os = "android")]
fn main() {}

#[cfg(target_os = "ios")]
fn main() -> Result<(), fandhe_render_egui::LaunchError> {
    fandhe_egui_proxy::ios_scene::install();
    fandhe_egui_proxy::ios::run(fandhe_example::build_app)
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() -> Result<(), fandhe_render_egui::LaunchError> {
    fandhe_egui_proxy::register_teardown_hooks();
    let name = fandhe_egui_proxy::selected_app_name().unwrap_or_else(|| "menu".to_string());
    fandhe_render_egui::launch::run_desktop("fandhe-example", None, move |lc| {
        let app = fandhe_example::build_app(&name, lc.egui_ctx()).expect("未登録の画面名");
        app
    })
}
