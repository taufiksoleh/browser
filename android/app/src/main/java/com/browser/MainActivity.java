package com.browser;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.graphics.Bitmap;
import android.net.Uri;
import android.os.Bundle;
import android.util.Log;
import android.view.KeyEvent;
import android.view.View;
import android.view.inputmethod.EditorInfo;
import android.webkit.WebChromeClient;
import android.webkit.WebResourceError;
import android.webkit.WebResourceRequest;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.EditText;
import android.widget.ImageButton;
import android.widget.ProgressBar;
import android.widget.Toast;

import java.io.File;
import java.io.FileWriter;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Locale;

public class MainActivity extends Activity {

    private WebView webView;
    private EditText urlBar;
    private ProgressBar progressBar;
    private ImageButton backButton;
    private ImageButton forwardButton;
    private ImageButton refreshButton;
    private ImageButton homeButton;

    private static final String DEFAULT_URL = "https://www.google.com";
    private static final String TAG = "MainActivity";
    private static final String CRASH_LOG_FILE = "crash.log";

    @SuppressLint("SetJavaScriptEnabled")
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Setup crash handler
        setupCrashHandler();

        // Check for previous crash
        checkForPreviousCrash();

        try {
            setContentView(R.layout.activity_main);

        // Initialize views
        webView = findViewById(R.id.webView);
        urlBar = findViewById(R.id.urlBar);
        progressBar = findViewById(R.id.progressBar);
        backButton = findViewById(R.id.backButton);
        forwardButton = findViewById(R.id.forwardButton);
        refreshButton = findViewById(R.id.refreshButton);
        homeButton = findViewById(R.id.homeButton);

        // Configure WebView settings
        WebSettings webSettings = webView.getSettings();
        webSettings.setJavaScriptEnabled(true);
        webSettings.setDomStorageEnabled(true);
        webSettings.setDatabaseEnabled(true);
        webSettings.setLoadWithOverviewMode(true);
        webSettings.setUseWideViewPort(true);
        webSettings.setBuiltInZoomControls(true);
        webSettings.setDisplayZoomControls(false);
        webSettings.setSupportZoom(true);
        webSettings.setAllowFileAccess(true);
        webSettings.setAllowContentAccess(true);
        webSettings.setCacheMode(WebSettings.LOAD_DEFAULT);
        webSettings.setMixedContentMode(WebSettings.MIXED_CONTENT_COMPATIBILITY_MODE);

        // Set user agent
        webSettings.setUserAgentString(webSettings.getUserAgentString() + " ChromiumBrowser/1.0");

        // Set WebViewClient
        webView.setWebViewClient(new WebViewClient() {
            @Override
            public void onPageStarted(WebView view, String url, Bitmap favicon) {
                super.onPageStarted(view, url, favicon);
                urlBar.setText(url);
                progressBar.setVisibility(View.VISIBLE);
                updateNavigationButtons();
            }

            @Override
            public void onPageFinished(WebView view, String url) {
                super.onPageFinished(view, url);
                progressBar.setVisibility(View.GONE);
                updateNavigationButtons();
            }

            @Override
            public void onReceivedError(WebView view, WebResourceRequest request, WebResourceError error) {
                super.onReceivedError(view, request, error);
                if (request.isForMainFrame()) {
                    showError("Error loading page: " + error.getDescription());
                }
            }

            @Override
            public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {
                String url = request.getUrl().toString();

                // Handle special URL schemes
                if (url.startsWith("tel:") || url.startsWith("mailto:") || url.startsWith("geo:")) {
                    Intent intent = new Intent(Intent.ACTION_VIEW, Uri.parse(url));
                    startActivity(intent);
                    return true;
                }

                return false;
            }
        });

        // Set WebChromeClient for progress updates
        webView.setWebChromeClient(new WebChromeClient() {
            @Override
            public void onProgressChanged(WebView view, int newProgress) {
                super.onProgressChanged(view, newProgress);
                progressBar.setProgress(newProgress);
            }

            @Override
            public void onReceivedTitle(WebView view, String title) {
                super.onReceivedTitle(view, title);
                // Could update window title here
            }
        });

        // Setup URL bar
        urlBar.setOnEditorActionListener((v, actionId, event) -> {
            if (actionId == EditorInfo.IME_ACTION_GO ||
                actionId == EditorInfo.IME_ACTION_SEARCH ||
                (event != null && event.getKeyCode() == KeyEvent.KEYCODE_ENTER)) {
                loadUrl(urlBar.getText().toString());
                return true;
            }
            return false;
        });

        // Setup navigation buttons
        backButton.setOnClickListener(v -> {
            if (webView.canGoBack()) {
                webView.goBack();
            }
        });

        forwardButton.setOnClickListener(v -> {
            if (webView.canGoForward()) {
                webView.goForward();
            }
        });

        refreshButton.setOnClickListener(v -> webView.reload());

        homeButton.setOnClickListener(v -> loadUrl(DEFAULT_URL));

            // Handle intent
            Intent intent = getIntent();
            String url = intent.getDataString();

            if (url != null && !url.isEmpty()) {
                loadUrl(url);
            } else if (savedInstanceState != null) {
                webView.restoreState(savedInstanceState);
            } else {
                loadUrl(DEFAULT_URL);
            }
        } catch (Exception e) {
            Log.e(TAG, "Error in onCreate", e);
            showCrashDialog("Initialization Error", e);
        }
    }

    private void setupCrashHandler() {
        final Thread.UncaughtExceptionHandler defaultHandler =
            Thread.getDefaultUncaughtExceptionHandler();

        Thread.setDefaultUncaughtExceptionHandler((thread, throwable) -> {
            try {
                // Log crash to file
                logCrashToFile(throwable);

                // Log to Android log
                Log.e(TAG, "Uncaught exception", throwable);
            } catch (Exception e) {
                Log.e(TAG, "Error logging crash", e);
            } finally {
                // Call default handler
                if (defaultHandler != null) {
                    defaultHandler.uncaughtException(thread, throwable);
                }
            }
        });
    }

    private void logCrashToFile(Throwable throwable) {
        try {
            File crashFile = new File(getFilesDir(), CRASH_LOG_FILE);
            FileWriter writer = new FileWriter(crashFile);

            SimpleDateFormat dateFormat = new SimpleDateFormat(
                "yyyy-MM-dd HH:mm:ss", Locale.US);
            writer.write("Crash Time: " + dateFormat.format(new Date()) + "\n");
            writer.write("Build Type: " + BuildConfig.BUILD_TYPE + "\n");
            writer.write("Version: " + BuildConfig.VERSION_NAME + "\n\n");

            StringWriter sw = new StringWriter();
            PrintWriter pw = new PrintWriter(sw);
            throwable.printStackTrace(pw);
            writer.write(sw.toString());

            writer.close();
        } catch (Exception e) {
            Log.e(TAG, "Failed to write crash log", e);
        }
    }

    private void checkForPreviousCrash() {
        try {
            File crashFile = new File(getFilesDir(), CRASH_LOG_FILE);
            if (crashFile.exists()) {
                // Read crash log
                java.io.FileReader reader = new java.io.FileReader(crashFile);
                java.io.BufferedReader bufferedReader = new java.io.BufferedReader(reader);
                StringBuilder crashLog = new StringBuilder();
                String line;
                while ((line = bufferedReader.readLine()) != null) {
                    crashLog.append(line).append("\n");
                }
                bufferedReader.close();

                // Show crash dialog
                final String crashMessage = crashLog.toString();
                runOnUiThread(() -> {
                    new AlertDialog.Builder(MainActivity.this)
                        .setTitle("Previous Crash Detected")
                        .setMessage("The app crashed previously. Details:\n\n" + crashMessage)
                        .setPositiveButton("OK", (dialog, which) -> {
                            // Delete crash log after showing
                            crashFile.delete();
                        })
                        .setNegativeButton("Copy Log", (dialog, which) -> {
                            android.content.ClipboardManager clipboard =
                                (android.content.ClipboardManager) getSystemService(CLIPBOARD_SERVICE);
                            android.content.ClipData clip =
                                android.content.ClipData.newPlainText("Crash Log", crashMessage);
                            clipboard.setPrimaryClip(clip);
                            Toast.makeText(MainActivity.this,
                                "Crash log copied to clipboard", Toast.LENGTH_SHORT).show();
                            crashFile.delete();
                        })
                        .setCancelable(false)
                        .show();
                });
            }
        } catch (Exception e) {
            Log.e(TAG, "Error checking for previous crash", e);
        }
    }

    private void showCrashDialog(String title, Exception e) {
        StringWriter sw = new StringWriter();
        PrintWriter pw = new PrintWriter(sw);
        e.printStackTrace(pw);
        String stackTrace = sw.toString();

        new AlertDialog.Builder(this)
            .setTitle(title)
            .setMessage("An error occurred:\n\n" + e.getMessage() + "\n\n" + stackTrace)
            .setPositiveButton("OK", null)
            .setNegativeButton("Copy", (dialog, which) -> {
                android.content.ClipboardManager clipboard =
                    (android.content.ClipboardManager) getSystemService(CLIPBOARD_SERVICE);
                android.content.ClipData clip =
                    android.content.ClipData.newPlainText("Error", stackTrace);
                clipboard.setPrimaryClip(clip);
                Toast.makeText(this, "Error copied to clipboard", Toast.LENGTH_SHORT).show();
            })
            .show();
    }

    private void loadUrl(String url) {
        // Add protocol if missing
        if (!url.startsWith("http://") && !url.startsWith("https://") && !url.startsWith("file://")) {
            // Check if it looks like a URL
            if (url.contains(".") && !url.contains(" ")) {
                url = "https://" + url;
            } else {
                // Treat as search query
                url = "https://www.google.com/search?q=" + Uri.encode(url);
            }
        }

        webView.loadUrl(url);
    }

    private void updateNavigationButtons() {
        backButton.setEnabled(webView.canGoBack());
        forwardButton.setEnabled(webView.canGoForward());
    }

    private void showError(String message) {
        new AlertDialog.Builder(this)
            .setTitle(R.string.error_title)
            .setMessage(message)
            .setPositiveButton(R.string.ok, null)
            .show();
    }

    @Override
    protected void onSaveInstanceState(Bundle outState) {
        super.onSaveInstanceState(outState);
        webView.saveState(outState);
    }

    @SuppressWarnings("deprecation")
    @Override
    public void onBackPressed() {
        if (webView.canGoBack()) {
            webView.goBack();
        } else {
            super.onBackPressed();
        }
    }

    @Override
    protected void onDestroy() {
        if (webView != null) {
            webView.destroy();
        }
        super.onDestroy();
    }
}
