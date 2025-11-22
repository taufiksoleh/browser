# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.

# Keep WebView classes
-keepclassmembers class fqcn.of.javascript.interface.for.webview {
   public *;
}

-keepattributes JavascriptInterface
-keepattributes *Annotation*

# Keep WebView JavaScript interfaces
-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}

# Keep custom WebView clients
-keep public class * extends android.webkit.WebViewClient
-keep public class * extends android.webkit.WebChromeClient

# Keep MainActivity
-keep public class com.browser.MainActivity {
    public *;
}

# General Android optimizations
-dontwarn android.webkit.**
-keep class android.webkit.** { *; }

# Remove logging in release builds
-assumenosideeffects class android.util.Log {
    public static *** d(...);
    public static *** v(...);
    public static *** i(...);
}
