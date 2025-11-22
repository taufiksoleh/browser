# Android Browser

A native Android web browser built using **Android WebView** with Chromium rendering engine.

## Features

- **Full Chromium Engine**: Complete HTML5, CSS3, and JavaScript support via Android WebView
- **Native Android UI**: Material Design interface with smooth animations
- **Modern Web Standards**: ES6+, WebGL, WebAssembly, Service Workers
- **Fast and Lightweight**: Optimized for performance and minimal APK size
- **Privacy Focused**: No tracking, no ads, pure browsing experience
- **Gesture Support**: Swipe navigation and modern mobile UX

## Requirements

- Android 7.0 (API level 24) or higher
- Android Studio Hedgehog (2023.1.1) or newer (for development)
- JDK 17 or newer

## Building

### Quick Start

```bash
cd android
./gradlew assembleDebug
```

The debug APK will be generated at:
```
app/build/outputs/apk/debug/app-debug.apk
```

### Building Release APK

```bash
cd android
./gradlew assembleRelease
```

The release APK (unsigned) will be generated at:
```
app/build/outputs/apk/release/app-release-unsigned.apk
```

### Build Variants

- **Debug**: Development build with debugging enabled
  - Package: `com.browser.debug`
  - Debuggable: Yes
  - Proguard: Disabled

- **Release**: Production build optimized for size and performance
  - Package: `com.browser`
  - Debuggable: No
  - Proguard: Enabled (code optimization and obfuscation)

## Installing

### Install Debug APK

```bash
cd android
./gradlew installDebug
```

Or install manually:
```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

### Install Release APK

For release builds, you need to sign the APK first. For testing purposes, you can install the debug-signed release:

```bash
adb install app/build/outputs/apk/release/app-release-unsigned.apk
```

## Running

After installation, launch the app:
- From app drawer: Look for "Browser" icon
- Via ADB:
  ```bash
  adb shell am start -n com.browser/.MainActivity
  ```

## Features Overview

### Navigation
- **Address Bar**: Type URLs or search terms (Google search integration)
- **Smart URL Detection**: Automatically adds `https://` for URLs
- **Search Integration**: Non-URL text triggers Google search
- **Navigation Buttons**: Back, Forward, Refresh, Home

### WebView Settings
- ✅ JavaScript enabled
- ✅ DOM storage enabled
- ✅ Database enabled
- ✅ Wide viewport support
- ✅ Built-in zoom controls
- ✅ Cache enabled
- ✅ Mixed content mode (compatibility)
- ✅ File and content access

### Security & Permissions

Required permissions:
- `INTERNET`: Network access for browsing
- `ACCESS_NETWORK_STATE`: Check network connectivity
- `WRITE_EXTERNAL_STORAGE`: Download files (Android 9 and below)
- `READ_EXTERNAL_STORAGE`: Access downloads (Android 9 and below)

## Development

### Project Structure

```
android/
├── app/
│   ├── src/main/
│   │   ├── java/com/browser/
│   │   │   └── MainActivity.java      # Main activity & WebView logic
│   │   ├── res/
│   │   │   ├── layout/
│   │   │   │   └── activity_main.xml  # Main UI layout
│   │   │   └── values/
│   │   │       ├── strings.xml        # String resources
│   │   │       └── styles.xml         # App theme
│   │   └── AndroidManifest.xml        # App manifest
│   ├── build.gradle                   # App-level Gradle config
│   └── proguard-rules.pro            # ProGuard rules
├── build.gradle                       # Project-level Gradle config
├── settings.gradle                    # Gradle settings
├── gradle.properties                  # Gradle properties
└── README.md                          # This file
```

### Opening in Android Studio

1. Open Android Studio
2. Select "Open an Existing Project"
3. Navigate to the `android/` directory
4. Click "OK"

Android Studio will automatically sync Gradle and index the project.

### Running in Emulator

1. Create an Android Virtual Device (AVD) in Android Studio
   - Minimum API level: 24 (Android 7.0)
   - Recommended: API level 33+ (Android 13+)

2. Run the app:
   ```bash
   ./gradlew installDebug
   ```

   Or use Android Studio's Run button (▶️)

### Testing on Physical Device

1. Enable Developer Options on your Android device
2. Enable USB Debugging
3. Connect device via USB
4. Verify device is detected:
   ```bash
   adb devices
   ```
5. Install and run:
   ```bash
   ./gradlew installDebug
   ```

## Customization

### Changing Default Homepage

Edit `MainActivity.java`:
```java
private static final String DEFAULT_URL = "https://your-homepage.com";
```

### Modifying UI Theme

Edit `app/src/main/res/values/styles.xml`:
```xml
<item name="android:colorPrimary">#YOUR_COLOR</item>
<item name="android:colorPrimaryDark">#YOUR_COLOR</item>
<item name="android:colorAccent">#YOUR_COLOR</item>
```

### Customizing WebView Settings

Edit `MainActivity.java` in the `onCreate` method:
```java
WebSettings webSettings = webView.getSettings();
webSettings.setJavaScriptEnabled(true);  // Enable/disable JavaScript
// Add more customizations...
```

## Troubleshooting

### Build Fails with "SDK not found"

Set `ANDROID_HOME` environment variable:
```bash
export ANDROID_HOME=/path/to/android/sdk
```

Or create `local.properties` in the `android/` directory:
```properties
sdk.dir=/path/to/android/sdk
```

### Gradle Sync Failed

1. Ensure JDK 17 is installed
2. Update Gradle wrapper:
   ```bash
   ./gradlew wrapper --gradle-version 8.2
   ```

### APK Not Installing

1. Uninstall previous version:
   ```bash
   adb uninstall com.browser
   adb uninstall com.browser.debug
   ```

2. Clear package manager cache:
   ```bash
   adb shell pm clear com.android.packageinstaller
   ```

## CI/CD Pipeline

The project includes GitHub Actions workflow for automated builds:

### Workflow Features

- ✅ Code quality checks
- ✅ Lint analysis
- ✅ Debug APK build
- ✅ Release APK build
- ✅ Unit tests
- ✅ Security scanning
- ✅ APK size analysis

### Artifacts

Each pipeline run produces:
- `browser-debug-apk`: Debug APK for testing (7-day retention)
- `browser-release-apk`: Release APK for distribution (30-day retention)
- `lint-results`: Lint analysis reports
- `test-results`: Unit test results

### Downloading APKs from CI

1. Go to repository's "Actions" tab
2. Click on the latest workflow run
3. Scroll to "Artifacts" section
4. Download the desired APK

## Performance

### APK Size

- Debug APK: ~3-4 MB
- Release APK: ~2-3 MB (with ProGuard optimization)

### Memory Usage

- Typical memory: 50-150 MB (depends on loaded pages)
- WebView is optimized by Android system

### Battery Impact

- Minimal when idle
- Background processes are suspended
- WebView uses hardware acceleration

## Compatibility

| Android Version | API Level | Status |
|-----------------|-----------|--------|
| Android 7.0     | 24        | ✅ Minimum supported |
| Android 8.0     | 26        | ✅ Fully supported |
| Android 9.0     | 28        | ✅ Fully supported |
| Android 10      | 29        | ✅ Fully supported |
| Android 11      | 30        | ✅ Fully supported |
| Android 12      | 31        | ✅ Fully supported |
| Android 13      | 33        | ✅ Fully supported |
| Android 14      | 34        | ✅ Target SDK |

## Known Limitations

- **No Extensions**: WebView doesn't support browser extensions
- **Limited DevTools**: No built-in developer tools (use Chrome Remote Debugging)
- **WebView Version**: Depends on system WebView version (auto-updated via Google Play)

## Security

### Best Practices Implemented

- ✅ ProGuard code obfuscation in release builds
- ✅ HTTPS by default
- ✅ No hardcoded secrets
- ✅ Content Security Policy support
- ✅ Secure WebView settings

### Reporting Security Issues

Please report security vulnerabilities responsibly.

## License

MIT License - See LICENSE file for details.

## Resources

- [Android WebView Documentation](https://developer.android.com/reference/android/webkit/WebView)
- [Android Developer Guide](https://developer.android.com/guide)
- [Chromium WebView](https://www.chromium.org/developers/how-tos/build-instructions-android-webview/)
