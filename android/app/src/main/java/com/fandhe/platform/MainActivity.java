package com.fandhe.platform;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;

import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

import com.google.androidgamesdk.GameActivity;

/**
 * 利用者アプリの最小 Activity。選んだ 5 機能（camera-capture / document-scan /
 * nfc-card-emulation / share-receive / home-widget）が Rust から JNI で呼ぶメソッドだけを持つ。
 *
 * <p>本体 {@code platforms/android} の MainActivity（約 2300 行）から、5 機能の委譲部分と
 * insets 通知だけを写した。JNI のシンボル名は {@code com.fandhe.platform.MainActivity} に固定。
 */
public class MainActivity extends GameActivity {
    static {
        // AndroidManifest の android.app.lib_name と Cargo.toml の [lib] name に一致させる。
        System.loadLibrary("fandhe_example");
    }

    // fandhe-egui-proxy の JNI エクスポート。画面の safe area（物理 px）を渡す。
    private static native void nativeOnWindowInsetsChanged(int left, int top, int right, int bottom);

    // registerForActivityResult は STARTED より前に呼ぶ契約のため、フィールド初期化子で作る。
    // 登録キーは生成順の連番なので、順序を入れ替えない。
    private final CameraCaptureHelper cameraCaptureHelper = new CameraCaptureHelper(this);
    private final DocumentScanHelper documentScanHelper = new DocumentScanHelper(this);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // コールドスタートの共有テキストを届ける。再生成・履歴からの再起動では届けない（二重配送回避）。
        if (savedInstanceState == null
                && (getIntent().getFlags() & Intent.FLAG_ACTIVITY_LAUNCHED_FROM_HISTORY) == 0) {
            ShareReceiveHelper.deliver(getIntent());
        }
        cameraCaptureHelper.onCreate(savedInstanceState);
        ViewCompat.setOnApplyWindowInsetsListener(getWindow().getDecorView(), (view, insets) -> {
            Insets bars = insets.getInsets(
                    WindowInsetsCompat.Type.systemBars() | WindowInsetsCompat.Type.displayCutout());
            nativeOnWindowInsetsChanged(bars.left, bars.top, bars.right, bars.bottom);
            return insets;
        });
    }

    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        setIntent(intent);
        ShareReceiveHelper.deliver(intent);
    }

    @Override
    protected void onSaveInstanceState(Bundle outState) {
        super.onSaveInstanceState(outState);
        cameraCaptureHelper.onSaveInstanceState(outState);
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        documentScanHelper.onDestroy();
    }

    // --- 以下は Rust 側（各機能 crate の src/android.rs）が JNI で参照する。名前と署名は固定。 ---

    // camera-capture: (J)V
    private void requestCameraCapture(long requestId) {
        cameraCaptureHelper.request(requestId);
    }

    // document-scan: (JI)V
    void requestDocumentScan(long requestId, int maxBytes) {
        documentScanHelper.scan(requestId, maxBytes);
    }

    // nfc-card-emulation: (Ljava/lang/String;)[J
    long[] nfcCardEmulationStatus(String aid) {
        return NfcCardEmulationHelper.status(getApplicationContext(), aid);
    }

    // home-widget: (Ljava/lang/String;)Z。保存は commit() で同期にし、再描画要求の失敗は保存成功を覆さない。
    private boolean homeWidgetSetText(String text) {
        Context app = getApplicationContext();
        boolean committed =
                app.getSharedPreferences(HomeWidgetProvider.PREFS_NAME, Context.MODE_PRIVATE)
                        .edit()
                        .putString(HomeWidgetProvider.KEY_TEXT, text)
                        .commit();
        if (!committed) {
            return false;
        }
        try {
            HomeWidgetProvider.requestUpdate(app);
        } catch (RuntimeException e) {
            android.util.Log.w("FandheHomeWidget", "requestUpdate failed: " + e.getClass().getSimpleName());
        }
        return true;
    }

    // home-widget: ()Ljava/lang/String;。未保存は null。
    private String homeWidgetCurrentText() {
        return HomeWidgetProvider.readText(getApplicationContext());
    }
}
