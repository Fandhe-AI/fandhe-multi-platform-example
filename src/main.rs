//! iOS・デスクトップの実行体。`lib.rs` の `#[fandhe::main]` が生成した入口を呼ぶ
//! （Android・Web では生成された `main` は何もしない）。

fn main() -> Result<(), fandhe::app::LaunchError> {
    fandhe_example::main()
}
