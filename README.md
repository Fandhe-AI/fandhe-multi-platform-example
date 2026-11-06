# fandhe-multi-platform-example

[fandhe-multi-platform](https://github.com/Fandhe-AI/fandhe-multi-platform) を外から使う最小アプリです。
本体を git 依存で取り込み、機能を 5 つだけ選んで iOS と Android で動かします。

## 選んだ機能

| 機能 crate | 画面 |
| --- | --- |
| `fandhe-cap-camera-capture` | カメラ撮影 |
| `fandhe-cap-document-scan` | 書類スキャン |
| `fandhe-cap-nfc-card-emulation` | NFC カードエミュレーション |
| `fandhe-cap-share-receive` | 共有の受け取り |
| `fandhe-cap-home-widget` | ホームウィジェット |

起動するとこの 5 画面へのリストが出ます。画面を開くと右上に「メニュー」ボタンが出ます。

## 構成

```text
Cargo.toml        本体を git 依存（rev 固定）で取り込む。[package.metadata.fandhe-app] に権限の説明文
build.rs          Apple のリンク指定の再出力
src/lib.rs        Android の android_main
src/main.rs       iOS・デスクトップの入口
src/screens.rs    画面の登録とメニュー
android/          Gradle の殻（MainActivity は 5 機能が呼ぶメソッドだけ）
scripts/          iOS シミュレータ向け .app の組み立て
gen/              cargo fandhe gen の出力（git 管理外）
```

## 前提

- Rust（`rust-toolchain.toml` の版）。ターゲット `aarch64-linux-android`・`x86_64-linux-android`・`aarch64-apple-ios-sim`
- 本体の submodule（`docs/spec`）の取得権限。cargo は git 依存の取得時に submodule も取るため、無いと失敗します
- `.cargo/config.toml` で `net.git-fetch-with-cli = true` にしています。git の資格情報を使うためです
- Android: JDK 17 以上、Android SDK（platform 35・build-tools 36）、NDK、`cargo install cargo-ndk`、Gradle 9.5 以上
- iOS: Xcode、起動済みのシミュレータ

## 手順

### 1. ネイティブ設定の生成

```sh
cargo install --git https://github.com/Fandhe-AI/fandhe-multi-platform --rev 27989c7a cargo-fandhe
cargo fandhe gen
```

`gen/` に Android の manifest・Gradle 設定と iOS の Info.plist が出ます。機能 crate を足したり外したりしたら再実行します。

### 2. ホストの確認

```sh
cargo check
```

### 3. Android

```sh
export ANDROID_HOME=<SDK の場所>
export ANDROID_NDK_HOME=<NDK の場所>
cargo ndk -t arm64-v8a -t x86_64 -o android/app/src/main/jniLibs build --lib --release
cd android && gradle :app:assembleDebug
adb install -r app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n com.fandhe.example/com.fandhe.platform.MainActivity
```

ABI は Gradle のプロパティ `fandheAndroidAbis`（既定 `arm64-v8a,x86_64`）で絞れます。

Vulkan を使います。Android 16（API 36）のエミュレータは、gfxstream の Vulkan ドライバが
`vkQueueSubmit` で落ちることがあります（本アプリのコードの外）。API 34 のエミュレータと実機を使ってください。

### 4. iOS シミュレータ

```sh
scripts/build-ios-sim.sh debug   # または release
xcrun simctl boot <デバイス ID>
xcrun simctl install booted build/ios-sim/FandheExample.app
xcrun simctl launch booted com.fandhe.example
```

起動画面は環境変数で選べます（既定はメニュー）。

```sh
SIMCTL_CHILD_FANDHE_PROXY_APP=document_scan_show xcrun simctl launch booted com.fandhe.example
```

画面名は `camera_capture`・`document_scan_show`・`nfc_card_emulation_show`・`share_receive_show`・`home_widget_show`・`menu` です。

## 制約

- 本体の入口 crate `fandhe` と `#[fandhe::main]`、`cargo fandhe run` / `build` は未提供のため、現在の構造（`fandhe-egui-proxy` の入口関数）に合わせています
- 実機での確認（カメラ・NFC・書類スキャン・共有）は未実施です
