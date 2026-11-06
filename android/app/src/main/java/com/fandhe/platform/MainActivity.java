package com.fandhe.platform;

import android.os.Bundle;

import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

import com.google.androidgamesdk.GameActivity;

/**
 * 利用者アプリの最小 Activity。Rust のライブラリを読み込み、画面の safe area を Rust へ渡すだけを持つ。
 *
 * <p>選んだ 5 機能（camera-capture / document-scan / nfc-card-emulation / share-receive / home-widget）の
 * JNI 入口と Activity への参加は、各機能 crate の Java（static 入口と非公開 provider）が持つため、
 * ここには何も書かない。機能の追加・削除は Cargo.toml の依存と {@code cargo fandhe gen} だけで済む。
 * JNI のシンボル名は {@code com.fandhe.platform.MainActivity} に固定。
 */
public class MainActivity extends GameActivity {
    static {
        // cdylib 名の正本は Cargo.toml の [lib] name。gen が FandheLibrary.NAME に写す。
        System.loadLibrary(FandheLibrary.NAME);
    }

    // fandhe-egui-proxy の JNI エクスポート。画面の safe area（物理 px）を渡す。
    private static native void nativeOnWindowInsetsChanged(int left, int top, int right, int bottom);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        ViewCompat.setOnApplyWindowInsetsListener(getWindow().getDecorView(), (view, insets) -> {
            Insets bars = insets.getInsets(
                    WindowInsetsCompat.Type.systemBars() | WindowInsetsCompat.Type.displayCutout());
            nativeOnWindowInsetsChanged(bars.left, bars.top, bars.right, bars.bottom);
            return insets;
        });
    }
}
