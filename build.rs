//! Apple（iOS / macOS）のリンク指定を `fandhe-egui-proxy` から受け取って出し直す。
//!
//! `cargo:rustc-link-arg` は出した package 自身の成果物にしか効かないため、最終成果物を作る
//! 本 crate が再出力する（本体 `fandhe-gallery` の `build.rs` と同じ）。Apple 以外では何もしない。

fn main() {
    fandhe_platform_apple::build::import_link_args("fandhe_egui_proxy");
}
