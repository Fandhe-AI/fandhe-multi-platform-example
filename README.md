# fandhe-multi-platform-example

[fandhe-multi-platform](https://github.com/Fandhe-AI/fandhe-multi-platform) を外から使う最小アプリです。
本体を git 依存で取り込み、機能を 5 つだけ選んで iOS と Android で動かします。

## 選んだ機能

| `fandhe` の feature | 画面 |
| --- | --- |
| `camera-capture` | カメラ撮影 |
| `document-scan` | 書類スキャン |
| `nfc-card-emulation` | NFC カードエミュレーション |
| `share-receive` | 共有の受け取り |
| `home-widget` | ホームウィジェット |

画面のボタンを端末機能へつなぐ effect ドライバは各機能 crate（`fandhe-cap-<id>`）にあるため、5 つの crate も直接依存に書いています。

起動するとこの 5 画面へのリストが出ます。画面を開くと右上に「メニュー」ボタンが出ます。

## 構成

```text
Cargo.toml        入口 crate fandhe を git 依存（rev 固定）で取り込み、機能を feature で選ぶ。[package.metadata.fandhe-app] に bundle id と権限の説明文
src/lib.rs        #[fandhe::main] を付けた起動画面の関数（メニュー）。各 OS の入口はここから生成される
src/main.rs       iOS・デスクトップの実行体（生成された main を呼ぶだけ）
android/          Gradle の殻（MainActivity はライブラリの読み込みだけ）
scripts/          iOS シミュレータ向け .app の組み立て
gen/              cargo fandhe gen の出力（git 管理外）
```

## 前提

- Rust（`rust-toolchain.toml` の版）。ターゲット `aarch64-linux-android`・`x86_64-linux-android`・`aarch64-apple-ios-sim`
- Android: JDK 17 以上、Android SDK（platform 35・build-tools 36）、NDK、`cargo install cargo-ndk`、Gradle 9.5 以上
- iOS: Xcode、起動済みのシミュレータ

## 手順

### 1. ネイティブ設定の生成

```sh
cargo install --git https://github.com/Fandhe-AI/fandhe-multi-platform --rev <Cargo.toml の rev> cargo-fandhe
cargo fandhe gen
```

`gen/` に Android の manifest・Gradle 設定・`FandheLibrary.java` と iOS の Info.plist が出ます。機能 crate を足したり外したりしたら再実行します。

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
API 34 のエミュレータで画面が黒いまま応答しなくなる場合は、`emulator -avd <名前> -gpu host` で起動します。

### 4. iOS シミュレータ

```sh
scripts/build-ios-sim.sh debug   # または release
xcrun simctl boot <デバイス ID>
xcrun simctl install booted build/ios-sim/FandheExample.app
xcrun simctl launch booted com.fandhe.example
```

## 制約

- `cargo fandhe run` / `build` は未提供のため、ビルドと起動は上の手順で行います
- 実機での確認（カメラ・NFC・書類スキャン・共有）は未実施です
