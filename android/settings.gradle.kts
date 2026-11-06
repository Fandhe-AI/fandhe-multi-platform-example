// 最小の Gradle プロジェクト。Rust の cdylib は cargo-ndk が `app/src/main/jniLibs` へ出す。
pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "fandhe-example-android"

include(":app")
