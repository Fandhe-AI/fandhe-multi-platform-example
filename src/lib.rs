//! 利用者の最小アプリ。fandhe-multi-platform の機能 crate を 5 つ選び、各 OS の入口へ画面の登録を渡す。
//!
//! - iOS・デスクトップ: `main.rs`（bin `fandhe-example`）
//! - Android: 本ファイルの `android_main`（cdylib `libfandhe_example.so`）
//!
//! Android のみ `unsafe_code` を `deny` へ緩め、`android_main` の `#[no_mangle]` 1 箇所にだけ許可する。
#![cfg_attr(not(target_os = "android"), forbid(unsafe_code))]
#![cfg_attr(target_os = "android", deny(unsafe_code))]

pub mod screens;
pub use screens::build_app;

/// Android のエントリポイント。`android-activity` がライブラリロード後にこの固定名で呼ぶ。
///
/// # Safety
/// この関数自体は `unsafe` 操作を含まない。`#[no_mangle]` は名前修飾を無効にするだけで、
/// `android_main` は `android-activity` の規約上の固定名。本 crate は cdylib として 1 つの
/// Android 実行単位にだけリンクされるため、他のシンボルと衝突しない。
#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[no_mangle]
fn android_main(app: fandhe_egui_proxy::android::AndroidApp) {
    // Activity 再生成で画面状態は作り直される。保留中の書類スキャン要求が残ると次の要求が
    // 飲み込まれるため、新しい画面状態を作る前に捨てる。
    fandhe_cap_document_scan::reset_pending_tracking();
    fandhe_egui_proxy::android::run(app, build_app);
}
