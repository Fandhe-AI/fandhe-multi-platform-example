// 利用者アプリ側の殻。Java・res・manifest・Gradle 依存は `cargo fandhe gen` の出力
// （`../../gen/android/`）から取り込み、ここには殻に固有の設定だけを書く。
plugins {
    id("com.android.application")
}

android {
    // JNI のシンボル名（Java_com_fandhe_platform_*）が固定のため、namespace は変えられない。
    // 利用者のアプリを区別するのは applicationId の方。
    namespace = "com.fandhe.platform"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.fandhe.example"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        val abiCsv = (project.findProperty("fandheAndroidAbis") as String?) ?: "arm64-v8a,x86_64"
        ndk {
            abiFilters += abiCsv.split(",").map { it.trim() }.filter { it.isNotEmpty() }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

// gen の出力（java.srcDirs・res.srcDirs・manifest の overlay・GameActivity 等の基盤と ML Kit の Gradle 依存）。
// 生成前は失敗させる。
val fandheGradle = rootProject.file("../gen/android/fandhe.gradle")
check(fandheGradle.exists()) { "gen/android/fandhe.gradle が無い。先に `cargo fandhe gen` を実行する" }
apply(from = fandheGradle)
