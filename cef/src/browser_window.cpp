// CEF Browser - Browser Window Implementation
#include "browser_window.h"
#include "browser_client.h"

#include "include/cef_app.h"
#include "include/wrapper/cef_helpers.h"

#if defined(OS_WIN)
#include <windows.h>
#endif

namespace {

// Global browser client instance
CefRefPtr<BrowserClient> g_browser_client;

}  // namespace

const char* BrowserWindow::kDefaultUrl = "https://www.google.com";

void BrowserWindow::Create() {
    CEF_REQUIRE_UI_THREAD();

    // Create browser client
    g_browser_client = new BrowserClient();

    CefWindowInfo window_info;
    CefBrowserSettings browser_settings;

    // Configure browser settings
    browser_settings.javascript_access_clipboard = STATE_ENABLED;
    browser_settings.javascript_dom_paste = STATE_ENABLED;
    browser_settings.local_storage = STATE_ENABLED;
    browser_settings.databases = STATE_ENABLED;
    browser_settings.webgl = STATE_ENABLED;

#if defined(OS_WIN)
    // Windows: Create a simple window
    WNDCLASSEX wcex = {0};
    wcex.cbSize = sizeof(WNDCLASSEX);
    wcex.style = CS_HREDRAW | CS_VREDRAW;
    wcex.lpfnWndProc = DefWindowProc;
    wcex.hInstance = GetModuleHandle(nullptr);
    wcex.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wcex.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    wcex.lpszClassName = "CEFBrowserWindow";
    RegisterClassEx(&wcex);

    HWND hwnd =
        CreateWindowEx(0, "CEFBrowserWindow", "CEF Browser", WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
                       CW_USEDEFAULT, CW_USEDEFAULT, kDefaultWidth, kDefaultHeight, nullptr,
                       nullptr, GetModuleHandle(nullptr), nullptr);

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    RECT rect;
    GetClientRect(hwnd, &rect);
    window_info.SetAsChild(hwnd, rect);

#else
    // Linux/macOS: With chrome_runtime, window is managed by Chrome
    // No special window_info setup needed
    (void)kDefaultWidth;
    (void)kDefaultHeight;
#endif

    // Create the browser
    CefBrowserHost::CreateBrowser(window_info, g_browser_client, kDefaultUrl, browser_settings,
                                  nullptr,  // extra_info
                                  nullptr   // request_context
    );
}

void BrowserWindow::Navigate(const std::string& url) {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        g_browser_client->GetBrowser()->GetMainFrame()->LoadURL(url);
    }
}

void BrowserWindow::GoBack() {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        if (g_browser_client->GetBrowser()->CanGoBack()) {
            g_browser_client->GetBrowser()->GoBack();
        }
    }
}

void BrowserWindow::GoForward() {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        if (g_browser_client->GetBrowser()->CanGoForward()) {
            g_browser_client->GetBrowser()->GoForward();
        }
    }
}

void BrowserWindow::Reload() {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        g_browser_client->GetBrowser()->Reload();
    }
}

void BrowserWindow::StopLoading() {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        g_browser_client->GetBrowser()->StopLoad();
    }
}

CefRefPtr<BrowserClient> BrowserWindow::GetClient() {
    return g_browser_client;
}

CefRefPtr<CefBrowser> BrowserWindow::GetBrowser() {
    if (g_browser_client) {
        return g_browser_client->GetBrowser();
    }
    return nullptr;
}
