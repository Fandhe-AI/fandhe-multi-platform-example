#!/usr/bin/env bash
# iOS シミュレータ向け .app を cargo の bin から手で組み立てる（Xcode プロジェクトは使わない）。
# 本体 scripts/ios/build-simulator-app.sh の最小版。Info.plist は `cargo fandhe gen` の出力
# （gen/ios/Info.plist）をそのまま使う。
#
# 使い方: scripts/build-ios-sim.sh [debug|release]   成果物: build/ios-sim/FandheExample.app
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

profile="${1:-release}"
target="aarch64-apple-ios-sim"
bin="fandhe-example"
app_name="FandheExample"
app="build/ios-sim/${app_name}.app"

[[ -f gen/ios/Info.plist ]] || { echo "gen/ios/Info.plist が無い。先に cargo fandhe gen を実行する" >&2; exit 1; }

rustup target add "${target}" >/dev/null
if [[ "${profile}" == "release" ]]; then cargo build --release --bin "${bin}" --target "${target}"
else cargo build --bin "${bin}" --target "${target}"; fi

rm -rf "${app}"; mkdir -p "${app}"
# gen の Info.plist は確定値（実行体名は bin 名・bundle id・版・MinimumOSVersion・UIDeviceFamily・UILaunchScreen）。
# 実行体は CFBundleExecutable に合わせて bin 名のまま置く。
cp "target/${target}/${profile}/${bin}" "${app}/${bin}"
cp gen/ios/Info.plist "${app}/Info.plist"

# gen は CFBundleSupportedPlatforms を出さない（シミュレータか実機かは組み立て時に決まる）。
/usr/libexec/PlistBuddy -c "Add :CFBundleSupportedPlatforms array" \
    -c "Add :CFBundleSupportedPlatforms:0 string iPhoneSimulator" "${app}/Info.plist"

codesign --sign - --force "${app}"
echo "OK: ${app}"
