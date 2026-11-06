//! 画面の登録。選んだ 5 機能の画面を名前で引き、起動時のメニューから切り替える。
//!
//! 本体 `fandhe-gallery` の `generated_screens!`（`effect` 区画）と同じ組み立てを、マクロなしで
//! 5 行だけ手で書いたもの。入口（`fandhe_egui_proxy::android::run` / `ios::run`）へ [`build_app`] を渡す。

use std::rc::Rc;

use fandhe_egui_proxy::app::{ProxyApp, ProxyAppDyn};
use fandhe_render_egui::app::InsetMarginSupplier;
use fandhe_render_egui::EguiApp;

/// メニューに並べる (画面名, 表示名)。
const SCREENS: [(&str, &str); 5] = [
    ("camera_capture", "カメラ撮影"),
    ("document_scan_show", "書類スキャン"),
    ("nfc_card_emulation_show", "NFC カードエミュレーション"),
    ("share_receive_show", "共有の受け取り"),
    ("home_widget_show", "ホームウィジェット"),
];

/// `effect` 区画相当: 状態駆動の画面 + 機能 crate の effect ドライバを `ProxyApp` に束ねる。
macro_rules! effect_app {
    ($ctx:expr, $screen:path, $driver:path) => {
        Box::new(ProxyApp::with_screen_fn($ctx, $screen, Default::default()).with_effect($driver))
            as Box<dyn ProxyAppDyn>
    };
}

/// 画面名から型消去済みアプリを作る。未登録名は `None`（入口が sample 画面へ倒す）。
pub fn build_app(name: &str, ctx: &egui::Context) -> Option<Box<dyn ProxyAppDyn>> {
    use fandhe_core::screens as s;
    Some(match name {
        "menu" => Box::new(MenuApp::new(ctx)),
        "camera_capture" => effect_app!(
            ctx,
            s::camera_capture::current_screen,
            fandhe_cap_camera_capture::camera_capture_effect
        ),
        "document_scan_show" => effect_app!(
            ctx,
            s::document_scan_show::current_screen,
            fandhe_cap_document_scan::document_scan_show_effect
        ),
        "nfc_card_emulation_show" => effect_app!(
            ctx,
            s::nfc_card_emulation_show::current_screen,
            fandhe_cap_nfc_card_emulation::nfc_card_emulation_show_effect
        ),
        "share_receive_show" => effect_app!(
            ctx,
            s::share_receive_show::current_screen,
            fandhe_cap_share_receive::share_receive_show_effect
        ),
        "home_widget_show" => effect_app!(
            ctx,
            s::home_widget_show::current_screen,
            fandhe_cap_home_widget::home_widget_show_effect
        ),
        _ => return None,
    })
}

/// 5 機能へのリストを出し、選んだ画面を子として包むトップ画面。
///
/// 子の表示中は右上に「メニュー」ボタンを重ねる。insets は入口（Android）が注入した供給関数を
/// 子へも配る（`Box<dyn Fn>` は複製できないため `Rc` で共有する）。
struct MenuApp {
    ctx: egui::Context,
    child: Option<Box<dyn ProxyAppDyn>>,
    inset: Option<Rc<InsetMarginSupplier>>,
}

impl MenuApp {
    fn new(ctx: &egui::Context) -> Self {
        // メニュー自身の日本語表示用。子（ProxyApp）も構築時に同じ関数を呼ぶ。
        fandhe_egui_proxy::font::install_cjk_font(ctx);
        Self {
            ctx: ctx.clone(),
            child: None,
            inset: None,
        }
    }

    fn inset_margin(&self, ctx: &egui::Context) -> egui::Margin {
        self.inset.as_ref().map_or(egui::Margin::ZERO, |f| f(ctx))
    }
}

impl EguiApp for MenuApp {
    fn update(&mut self, ctx: &egui::Context) {
        if let Some(child) = self.child.as_mut() {
            child.update(ctx);
            let top = self.inset_margin(ctx).top as f32;
            let mut back = false;
            egui::Area::new(egui::Id::new("menu_back"))
                .anchor(egui::Align2::RIGHT_TOP, [-8.0, top + 8.0])
                .order(egui::Order::Foreground)
                .show(ctx, |ui| back = ui.button("メニュー").clicked());
            if back {
                self.child = None;
            }
            return;
        }
        let base = egui::Frame::central_panel(&ctx.style());
        let frame = base.inner_margin(base.inner_margin + self.inset_margin(ctx));
        let mut picked = None;
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.heading("fandhe example");
            ui.label("選んだ 5 機能");
            ui.add_space(12.0);
            for (name, label) in SCREENS {
                if ui
                    .add_sized([ui.available_width(), 48.0], egui::Button::new(label))
                    .clicked()
                {
                    picked = Some(name);
                }
            }
        });
        if let Some(name) = picked {
            if let Some(mut child) = build_app(name, &self.ctx) {
                if let Some(f) = &self.inset {
                    let f = Rc::clone(f);
                    child.set_inset_margin_supplier(Box::new(move |c| f(c)));
                }
                self.child = Some(child);
            }
        }
    }
}

impl ProxyAppDyn for MenuApp {
    fn set_inset_margin_supplier(&mut self, supplier: InsetMarginSupplier) {
        self.inset = Some(Rc::new(supplier));
    }

    fn set_autoshot(
        &mut self,
        _output_path: std::path::PathBuf,
        _save_failed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) {
    }
}
