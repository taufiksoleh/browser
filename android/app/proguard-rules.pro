# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.

# Keep source file names and line numbers for crash reports
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile

# Keep all annotations
-keepattributes *Annotation*
-keepattributes Signature
-keepattributes Exceptions

# Keep WebView classes
-keepclassmembers class fqcn.of.javascript.interface.for.webview {
   public *;
}

-keepattributes JavascriptInterface

# Keep WebView JavaScript interfaces
-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}

# Keep custom WebView clients
-keep public class * extends android.webkit.WebViewClient {
    <methods>;
}
-keep public class * extends android.webkit.WebChromeClient {
    <methods>;
}

# Keep MainActivity and all its methods
-keep public class com.browser.MainActivity {
    public <methods>;
    protected <methods>;
    private <methods>;
}

# Keep BuildConfig
-keep class com.browser.BuildConfig { *; }

# Keep all view-related methods (used by XML layouts)
-keepclassmembers class * extends android.app.Activity {
    public void *(android.view.View);
}

# Keep view constructors (used by XML layouts)
-keepclasseswithmembers class * {
    public <init>(android.content.Context, android.util.AttributeSet);
}
-keepclasseswithmembers class * {
    public <init>(android.content.Context, android.util.AttributeSet, int);
}

# Keep WebView and related classes
-dontwarn android.webkit.**
-keep class android.webkit.** { *; }
-keepclassmembers class android.webkit.** { *; }

# Keep AndroidX classes
-keep class androidx.** { *; }
-keep interface androidx.** { *; }
-dontwarn androidx.**

# Keep Google Material components
-keep class com.google.android.material.** { *; }
-dontwarn com.google.android.material.**

# Keep Android support library classes
-keep class android.support.** { *; }
-dontwarn android.support.**

# Keep AppCompat resources
-keep class androidx.appcompat.widget.** { *; }
-keep class androidx.appcompat.app.** { *; }

# Keep R class and all its inner classes
-keepclassmembers class **.R$* {
    public static <fields>;
}
-keep class **.R$*
-keep class **.R { *; }

# Keep all resources
-keep class **.R$drawable { *; }
-keep class **.R$layout { *; }
-keep class **.R$string { *; }
-keep class **.R$style { *; }
-keep class **.R$styleable { *; }
-keep class **.R$attr { *; }
-keep class **.R$color { *; }
-keep class **.R$dimen { *; }
-keep class **.R$id { *; }
-keep class **.R$mipmap { *; }

# Keep vector drawable classes
-keep class androidx.vectordrawable.** { *; }
-dontwarn androidx.vectordrawable.**

# Keep Bundle savedInstanceState classes
-keepclassmembers class * implements android.os.Parcelable {
    public static final android.os.Parcelable$Creator *;
}

# Keep Serializable classes
-keepclassmembers class * implements java.io.Serializable {
    static final long serialVersionUID;
    private static final java.io.ObjectStreamField[] serialPersistentFields;
    private void writeObject(java.io.ObjectOutputStream);
    private void readObject(java.io.ObjectInputStream);
    java.lang.Object writeReplace();
    java.lang.Object readResolve();
}

# Keep enum classes
-keepclassmembers enum * {
    public static **[] values();
    public static ** valueOf(java.lang.String);
}

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep AlertDialog and related classes
-keep class androidx.appcompat.app.AlertDialog { *; }
-keep class androidx.appcompat.app.AlertDialog$Builder { *; }

# Don't obfuscate (helps with debugging)
# Comment this out for smaller APK size, but keep it for debugging
-dontobfuscate

# Remove verbose logging in release builds
-assumenosideeffects class android.util.Log {
    public static *** d(...);
    public static *** v(...);
}
