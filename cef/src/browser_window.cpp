// CEF Browser - Browser Window Implementation
#include "browser_window.h"
#include "browser_client.h"

#include "include/cef_app.h"
#include "include/wrapper/cef_helpers.h"

#if defined(OS_LINUX)
#include <gtk/gtk.h>
#include <X11/Xlib.h>
#endif

#if defined(OS_WIN)
#include <windows.h>
#endif

namespace {

// Global browser client instance
CefRefPtr<BrowserClient> g_browser_client;

#if defined(OS_LINUX)
// GTK signal handlers
void OnDestroy(GtkWidget* widget, gpointer data) {
    CefQuitMessageLoop();
}

gboolean OnDelete(GtkWidget* widget, GdkEvent* event, gpointer data) {
    if (g_browser_client && g_browser_client->GetBrowser()) {
        g_browser_client->GetBrowser()->GetHost()->CloseBrowser(false);
    }
    return TRUE;  // Do not destroy the window until CEF closes
}
#endif

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

#if defined(OS_LINUX)
    // Initialize GTK
    gtk_init(nullptr, nullptr);

    // Create main window
    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "CEF Browser");
    gtk_window_set_default_size(GTK_WINDOW(window), kDefaultWidth, kDefaultHeight);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

    // Create vertical box for layout
    GtkWidget* vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_container_add(GTK_CONTAINER(window), vbox);

    // Create toolbar
    GtkWidget* toolbar = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);
    gtk_widget_set_margin_start(toolbar, 5);
    gtk_widget_set_margin_end(toolbar, 5);
    gtk_widget_set_margin_top(toolbar, 5);
    gtk_widget_set_margin_bottom(toolbar, 5);

    // Back button
    GtkWidget* back_btn = gtk_button_new_with_label("←");
    gtk_widget_set_tooltip_text(back_btn, "Go Back (Alt+Left)");
    gtk_box_pack_start(GTK_BOX(toolbar), back_btn, FALSE, FALSE, 0);

    // Forward button
    GtkWidget* forward_btn = gtk_button_new_with_label("→");
    gtk_widget_set_tooltip_text(forward_btn, "Go Forward (Alt+Right)");
    gtk_box_pack_start(GTK_BOX(toolbar), forward_btn, FALSE, FALSE, 0);

    // Reload button
    GtkWidget* reload_btn = gtk_button_new_with_label("↻");
    gtk_widget_set_tooltip_text(reload_btn, "Reload (Ctrl+R)");
    gtk_box_pack_start(GTK_BOX(toolbar), reload_btn, FALSE, FALSE, 0);

    // Home button
    GtkWidget* home_btn = gtk_button_new_with_label("⌂");
    gtk_widget_set_tooltip_text(home_btn, "Home");
    gtk_box_pack_start(GTK_BOX(toolbar), home_btn, FALSE, FALSE, 0);

    // URL entry
    GtkWidget* url_entry = gtk_entry_new();
    gtk_entry_set_text(GTK_ENTRY(url_entry), kDefaultUrl);
    gtk_widget_set_tooltip_text(url_entry, "Enter URL (Ctrl+L to focus)");
    gtk_box_pack_start(GTK_BOX(toolbar), url_entry, TRUE, TRUE, 0);

    // Add toolbar to vbox
    gtk_box_pack_start(GTK_BOX(vbox), toolbar, FALSE, FALSE, 0);

    // Create browser container
    GtkWidget* browser_container = gtk_drawing_area_new();
    gtk_widget_set_vexpand(browser_container, TRUE);
    gtk_widget_set_hexpand(browser_container, TRUE);
    gtk_box_pack_start(GTK_BOX(vbox), browser_container, TRUE, TRUE, 0);

    // Connect signals
    g_signal_connect(window, "destroy", G_CALLBACK(OnDestroy), nullptr);
    g_signal_connect(window, "delete-event", G_CALLBACK(OnDelete), nullptr);

    // Show window
    gtk_widget_show_all(window);

    // Get the native X11 window handle
    GdkWindow* gdk_window = gtk_widget_get_window(browser_container);
    while (!gdk_window) {
        gtk_main_iteration();
        gdk_window = gtk_widget_get_window(browser_container);
    }

    // Configure window info for Linux/X11
    XID xid = GDK_WINDOW_XID(gdk_window);
    window_info.SetAsChild(xid, CefRect(0, 0, kDefaultWidth, kDefaultHeight - 50));

#elif defined(OS_WIN)
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

    HWND hwnd = CreateWindowEx(
        0, "CEFBrowserWindow", "CEF Browser",
        WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
        CW_USEDEFAULT, CW_USEDEFAULT,
        kDefaultWidth, kDefaultHeight,
        nullptr, nullptr, GetModuleHandle(nullptr), nullptr);

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    RECT rect;
    GetClientRect(hwnd, &rect);
    window_info.SetAsChild(hwnd, rect);

#elif defined(OS_MAC)
    // macOS: Use views framework (requires additional setup)
    // For simplicity, create a basic window
    window_info.SetAsWindowless(kNullWindowHandle);

#else
    // Fallback: Create popup window
    window_info.SetAsPopup(kNullWindowHandle, "CEF Browser");
#endif

    // Create the browser
    CefBrowserHost::CreateBrowser(
        window_info,
        g_browser_client,
        kDefaultUrl,
        browser_settings,
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

CefRefPtr<CefBrowser> BrowserWindow::GetBrowser() {
    if (g_browser_client) {
        return g_browser_client->GetBrowser();
    }
    return nullptr;
}
