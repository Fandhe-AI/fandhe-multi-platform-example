#!/usr/bin/env bash
# iOS シミュレータ向け .app を cargo の bin から手で組み立てる（Xcode プロジェクトは使わない）。
# 本体 scripts/ios/build-simulator-app.sh の最小版。Info.plist は `cargo fandhe gen` の出力
# （gen/ios/Info.plist）を使い、Xcode が埋める変数をここで置換する。
#
# 使い方: scripts/build-ios-sim.sh [debug|release]   成果物: build/ios-sim/FandheExample.app
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

profile="${1:-release}"
target="aarch64-apple-ios-sim"
bin="fandhe-example"
app_name="FandheExample"
bundle_id="com.fandhe.example"
app="build/ios-sim/${app_name}.app"

[[ -f gen/ios/Info.plist ]] || { echo "gen/ios/Info.plist が無い。先に cargo fandhe gen を実行する" >&2; exit 1; }

rustup target add "${target}" >/dev/null
if [[ "${profile}" == "release" ]]; then cargo build --release --bin "${bin}" --target "${target}"
else cargo build --bin "${bin}" --target "${target}"; fi

rm -rf "${app}"; mkdir -p "${app}"
cp "target/${target}/${profile}/${bin}" "${app}/${app_name}"

# gen の Info.plist は Xcode 変数（$(EXECUTABLE_NAME) 等）を含む。Xcode を通さないため自前で置換する。
sed -e "s/\$(EXECUTABLE_NAME)/${app_name}/" \
    -e "s/\$(PRODUCT_BUNDLE_IDENTIFIER)/${bundle_id}/" \
    -e "s/\$(PRODUCT_NAME)/${app_name}/" \
    -e "s/\$(PRODUCT_BUNDLE_PACKAGE_TYPE)/APPL/" \
    -e "s/\$(MARKETING_VERSION)/0.1.0/" \
    -e "s/\$(CURRENT_PROJECT_VERSION)/1/" \
    gen/ios/Info.plist > "${app}/Info.plist"

# gen は端末種別・最小 OS・起動画面を出さない。無いとシミュレータが互換モード（黒帯）で起動する。
pb=/usr/libexec/PlistBuddy
"${pb}" -c "Add :UIDeviceFamily array" -c "Add :UIDeviceFamily:0 integer 1" -c "Add :UIDeviceFamily:1 integer 2" \
        -c "Add :MinimumOSVersion string 17.0" -c "Add :UILaunchScreen dict" \
        -c "Add :CFBundleSupportedPlatforms array" -c "Add :CFBundleSupportedPlatforms:0 string iPhoneSimulator" \
        "${app}/Info.plist"

codesign --sign - --force "${app}"
echo "OK: ${app}"
