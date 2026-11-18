//! 利用者の最小アプリ。fandhe-multi-platform の端末機能を 5 つ選び、メニューから画面を切り替える。
//!
//! 各 OS の入口は `#[fandhe::main]` が [`app`] から生成する。
//!
//! - iOS・デスクトップ: `main.rs`（bin `fandhe-example`）が生成された `main` を呼ぶ
//! - Android: 生成された `android_main`（cdylib `libfandhe_example.so`）
//!
//! 選んだ機能の起動前の登録・終了処理・Activity 再生成時の破棄は入口（`fandhe` の feature）が行う。
//!
//! Android のみ `unsafe_code` を `deny` へ緩める（生成される `android_main` は `no_mangle` を伴う）。
#![cfg_attr(not(target_os = "android"), forbid(unsafe_code))]
#![cfg_attr(target_os = "android", deny(unsafe_code))]

use fandhe::app::{Menu, ProxyApp, ProxyAppDyn};
use fandhe::egui;
use fandhe::screens as s;

/// 状態駆動の画面と、機能 crate の effect ドライバ（画面のボタン → 端末機能）を束ねる。
macro_rules! effect_app {
    ($screen:path, $driver:path) => {
        |ctx: &egui::Context| -> Box<dyn ProxyAppDyn> {
            Box::new(ProxyApp::with_screen_fn(ctx, $screen, Default::default()).with_effect($driver))
        }
    };
}

/// 起動画面。選んだ 5 機能へのメニューを出す。画面を開くと右上に「メニュー」ボタンが出る。
#[fandhe::main]
fn app(ctx: &egui::Context) -> Box<dyn ProxyAppDyn> {
    Box::new(
        Menu::new(ctx, "fandhe example")
            .screen(
                "カメラ撮影",
                effect_app!(
                    s::camera_capture::current_screen,
                    fandhe_cap_camera_capture::driver::camera_capture_effect
                ),
            )
            .screen(
                "書類スキャン",
                effect_app!(
                    s::document_scan_show::current_screen,
                    fandhe_cap_document_scan::driver::document_scan_show_effect
                ),
            )
            .screen(
                "NFC カードエミュレーション",
                effect_app!(
                    s::nfc_card_emulation_show::current_screen,
                    fandhe_cap_nfc_card_emulation::driver::nfc_card_emulation_show_effect
                ),
            )
            .screen(
                "共有の受け取り",
                effect_app!(
                    s::share_receive_show::current_screen,
                    fandhe_cap_share_receive::driver::share_receive_show_effect
                ),
            )
            .screen(
                "ホームウィジェット",
                effect_app!(
                    s::home_widget_show::current_screen,
                    fandhe_cap_home_widget::driver::home_widget_show_effect
                ),
            ),
    )
}
