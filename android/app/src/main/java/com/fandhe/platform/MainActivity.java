package com.fandhe.platform;

import com.google.androidgamesdk.GameActivity;

/**
 * 利用者アプリの最小 Activity。Rust のライブラリを読み込むだけを持つ。
 *
 * <p>選んだ機能の JNI 入口と Activity への参加は各機能 crate の Java（static 入口と非公開 provider）が、
 * 画面の safe area（window insets）の中継は共通層（{@code HostInsets}・{@code HostInsetsLifecycle}）が持つため、
 * ここには何も書かない。機能の追加・削除は Cargo.toml の feature と {@code cargo fandhe gen} だけで済む。
 */
public class MainActivity extends GameActivity {
    static {
        // cdylib 名の正本は Cargo.toml の [lib] name。gen が FandheLibrary.NAME に写す。
        System.loadLibrary(FandheLibrary.NAME);
    }
}
