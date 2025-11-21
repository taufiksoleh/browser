// CEF Browser - Browser Client Implementation
#include "browser_client.h"
#include "resource_util.h"

#include <sstream>
#include <string>

#include "include/cef_app.h"
#include "include/cef_parser.h"
#include "include/wrapper/cef_helpers.h"

int BrowserClient::browser_count_ = 0;

// Custom context menu IDs
enum ContextMenuId {
    MENU_ID_VIEW_SOURCE = MENU_ID_USER_FIRST,
    MENU_ID_OPEN_DEVTOOLS,
    MENU_ID_CLOSE_DEVTOOLS,
    MENU_ID_RELOAD_PAGE,
    MENU_ID_COPY_URL,
};

BrowserClient::BrowserClient() : is_closing_(false) {}

BrowserClient::~BrowserClient() {}

// ============================================================================
// CefLifeSpanHandler methods
// ============================================================================

bool BrowserClient::OnBeforePopup(
    CefRefPtr<CefBrowser> browser,
    CefRefPtr<CefFrame> frame,
    const CefString& target_url,
    const CefString& target_frame_name,
    CefLifeSpanHandler::WindowOpenDisposition target_disposition,
    bool user_gesture,
    const CefPopupFeatures& popupFeatures,
    CefWindowInfo& windowInfo,
    CefRefPtr<CefClient>& client,
    CefBrowserSettings& settings,
    CefRefPtr<CefDictionaryValue>& extra_info,
    bool* no_javascript_access) {
    CEF_REQUIRE_UI_THREAD();

    // Open popups in a new tab instead of a new window
    if (target_disposition == CEF_WOD_NEW_POPUP ||
        target_disposition == CEF_WOD_NEW_WINDOW) {
        // Load the URL in the current browser
        browser->GetMainFrame()->LoadURL(target_url);
        return true;  // Cancel popup
    }

    return false;  // Allow popup
}

void BrowserClient::OnAfterCreated(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();

    browser_list_.push_back(browser);
    browser_count_++;

    if (!browser_) {
        browser_ = browser;
    }
}

bool BrowserClient::DoClose(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();

    // Set closing flag
    if (browser_list_.size() == 1) {
        is_closing_ = true;
    }

    // Allow the close
    return false;
}

void BrowserClient::OnBeforeClose(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();

    // Remove from list
    for (auto it = browser_list_.begin(); it != browser_list_.end(); ++it) {
        if ((*it)->IsSame(browser)) {
            browser_list_.erase(it);
            break;
        }
    }

    browser_count_--;

    if (browser_list_.empty()) {
        browser_ = nullptr;
        // Quit the message loop when all browsers have closed
        CefQuitMessageLoop();
    }
}

// ============================================================================
// CefDisplayHandler methods
// ============================================================================

void BrowserClient::OnTitleChange(CefRefPtr<CefBrowser> browser,
                                   const CefString& title) {
    CEF_REQUIRE_UI_THREAD();

    // Update window title
    std::string window_title = title.ToString();
    if (window_title.empty()) {
        window_title = "CEF Browser";
    } else {
        window_title += " - CEF Browser";
    }

#if defined(OS_LINUX)
    // Update X11 window title
    // This would require X11 API calls
#elif defined(OS_WIN)
    // Update Windows window title
    HWND hwnd = browser->GetHost()->GetWindowHandle();
    if (hwnd) {
        SetWindowTextA(hwnd, window_title.c_str());
    }
#endif
}

void BrowserClient::OnAddressChange(CefRefPtr<CefBrowser> browser,
                                     CefRefPtr<CefFrame> frame,
                                     const CefString& url) {
    CEF_REQUIRE_UI_THREAD();

    if (frame->IsMain()) {
        // Address has changed - could update address bar UI here
        std::string current_url = url.ToString();
        // UI update would go here
    }
}

void BrowserClient::OnFullscreenModeChange(CefRefPtr<CefBrowser> browser,
                                            bool fullscreen) {
    CEF_REQUIRE_UI_THREAD();

    // Handle fullscreen mode change
    // UI update would go here
}

bool BrowserClient::OnConsoleMessage(CefRefPtr<CefBrowser> browser,
                                      cef_log_severity_t level,
                                      const CefString& message,
                                      const CefString& source,
                                      int line) {
    // Log console messages for debugging
    std::string level_str;
    switch (level) {
        case LOGSEVERITY_DEBUG:
            level_str = "DEBUG";
            break;
        case LOGSEVERITY_INFO:
            level_str = "INFO";
            break;
        case LOGSEVERITY_WARNING:
            level_str = "WARN";
            break;
        case LOGSEVERITY_ERROR:
            level_str = "ERROR";
            break;
        default:
            level_str = "LOG";
    }

    // Could log to file or console here
    return false;  // Allow default handling
}

// ============================================================================
// CefLoadHandler methods
// ============================================================================

void BrowserClient::OnLoadingStateChange(CefRefPtr<CefBrowser> browser,
                                          bool isLoading,
                                          bool canGoBack,
                                          bool canGoForward) {
    CEF_REQUIRE_UI_THREAD();

    // Update loading indicator and navigation buttons
    // UI update would go here
}

void BrowserClient::OnLoadStart(CefRefPtr<CefBrowser> browser,
                                 CefRefPtr<CefFrame> frame,
                                 TransitionType transition_type) {
    CEF_REQUIRE_UI_THREAD();

    if (frame->IsMain()) {
        // Page load started
    }
}

void BrowserClient::OnLoadEnd(CefRefPtr<CefBrowser> browser,
                               CefRefPtr<CefFrame> frame,
                               int httpStatusCode) {
    CEF_REQUIRE_UI_THREAD();

    if (frame->IsMain()) {
        // Page load completed
    }
}

void BrowserClient::OnLoadError(CefRefPtr<CefBrowser> browser,
                                 CefRefPtr<CefFrame> frame,
                                 ErrorCode errorCode,
                                 const CefString& errorText,
                                 const CefString& failedUrl) {
    CEF_REQUIRE_UI_THREAD();

    // Don't display an error for cancelled requests
    if (errorCode == ERR_ABORTED) {
        return;
    }

    // Display error page
    std::stringstream ss;
    ss << "<html><head><title>Load Error</title>"
       << "<style>"
       << "body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; "
       << "       padding: 50px; text-align: center; background: #f5f5f5; }"
       << "h1 { color: #333; }"
       << ".error-code { color: #666; font-size: 14px; }"
       << ".url { color: #0066cc; word-break: break-all; }"
       << ".retry-btn { margin-top: 20px; padding: 10px 20px; "
       << "             background: #0066cc; color: white; border: none; "
       << "             border-radius: 5px; cursor: pointer; font-size: 16px; }"
       << ".retry-btn:hover { background: #0055aa; }"
       << "</style></head><body>"
       << "<h1>This page isn't working</h1>"
       << "<p class='error-code'>Error: " << errorText.ToString()
       << " (" << errorCode << ")</p>"
       << "<p class='url'>" << failedUrl.ToString() << "</p>"
       << "<button class='retry-btn' onclick='location.reload()'>Retry</button>"
       << "</body></html>";

    frame->LoadURL(GetDataURI(ss.str(), "text/html"));
}

// ============================================================================
// CefRequestHandler methods
// ============================================================================

bool BrowserClient::OnBeforeBrowse(CefRefPtr<CefBrowser> browser,
                                    CefRefPtr<CefFrame> frame,
                                    CefRefPtr<CefRequest> request,
                                    bool user_gesture,
                                    bool is_redirect) {
    CEF_REQUIRE_UI_THREAD();

    // Allow all navigation by default
    return false;
}

// ============================================================================
// CefContextMenuHandler methods
// ============================================================================

void BrowserClient::OnBeforeContextMenu(CefRefPtr<CefBrowser> browser,
                                         CefRefPtr<CefFrame> frame,
                                         CefRefPtr<CefContextMenuParams> params,
                                         CefRefPtr<CefMenuModel> model) {
    CEF_REQUIRE_UI_THREAD();

    // Add separator
    if (model->GetCount() > 0) {
        model->AddSeparator();
    }

    // Add custom menu items
    model->AddItem(MENU_ID_RELOAD_PAGE, "Reload");
    model->AddItem(MENU_ID_VIEW_SOURCE, "View Page Source");
    model->AddSeparator();
    model->AddItem(MENU_ID_OPEN_DEVTOOLS, "Inspect Element");
    model->AddItem(MENU_ID_COPY_URL, "Copy URL");
}

bool BrowserClient::OnContextMenuCommand(CefRefPtr<CefBrowser> browser,
                                          CefRefPtr<CefFrame> frame,
                                          CefRefPtr<CefContextMenuParams> params,
                                          int command_id,
                                          EventFlags event_flags) {
    CEF_REQUIRE_UI_THREAD();

    switch (command_id) {
        case MENU_ID_VIEW_SOURCE:
            browser->GetMainFrame()->ViewSource();
            return true;

        case MENU_ID_OPEN_DEVTOOLS: {
            CefWindowInfo windowInfo;
            CefBrowserSettings settings;
#if defined(OS_WIN)
            windowInfo.SetAsPopup(nullptr, "DevTools");
#endif
            browser->GetHost()->ShowDevTools(windowInfo, nullptr, settings,
                                              CefPoint());
            return true;
        }

        case MENU_ID_CLOSE_DEVTOOLS:
            browser->GetHost()->CloseDevTools();
            return true;

        case MENU_ID_RELOAD_PAGE:
            browser->Reload();
            return true;

        case MENU_ID_COPY_URL:
            // Copy URL to clipboard
            frame->Copy();
            return true;

        default:
            return false;
    }
}

// ============================================================================
// CefKeyboardHandler methods
// ============================================================================

bool BrowserClient::OnPreKeyEvent(CefRefPtr<CefBrowser> browser,
                                   const CefKeyEvent& event,
                                   CefEventHandle os_event,
                                   bool* is_keyboard_shortcut) {
    CEF_REQUIRE_UI_THREAD();

    // Handle keyboard shortcuts
    if (event.type == KEYEVENT_RAWKEYDOWN) {
        bool ctrl_down = (event.modifiers & EVENTFLAG_CONTROL_DOWN) != 0;
        bool shift_down = (event.modifiers & EVENTFLAG_SHIFT_DOWN) != 0;

        // Ctrl+R or F5: Reload
        if ((ctrl_down && event.windows_key_code == 'R') ||
            event.windows_key_code == 0x74) {  // F5
            if (shift_down) {
                browser->ReloadIgnoreCache();
            } else {
                browser->Reload();
            }
            return true;
        }

        // Ctrl+Shift+I or F12: DevTools
        if ((ctrl_down && shift_down && event.windows_key_code == 'I') ||
            event.windows_key_code == 0x7B) {  // F12
            CefWindowInfo windowInfo;
            CefBrowserSettings settings;
            browser->GetHost()->ShowDevTools(windowInfo, nullptr, settings,
                                              CefPoint());
            return true;
        }

        // Ctrl+L: Focus address bar (would need UI integration)
        if (ctrl_down && event.windows_key_code == 'L') {
            *is_keyboard_shortcut = true;
            return false;
        }

        // Ctrl+W: Close tab
        if (ctrl_down && event.windows_key_code == 'W') {
            browser->GetHost()->CloseBrowser(false);
            return true;
        }

        // Alt+Left: Back
        if ((event.modifiers & EVENTFLAG_ALT_DOWN) &&
            event.windows_key_code == 0x25) {  // Left arrow
            if (browser->CanGoBack()) {
                browser->GoBack();
            }
            return true;
        }

        // Alt+Right: Forward
        if ((event.modifiers & EVENTFLAG_ALT_DOWN) &&
            event.windows_key_code == 0x27) {  // Right arrow
            if (browser->CanGoForward()) {
                browser->GoForward();
            }
            return true;
        }
    }

    return false;
}

bool BrowserClient::OnKeyEvent(CefRefPtr<CefBrowser> browser,
                                const CefKeyEvent& event,
                                CefEventHandle os_event) {
    return false;
}

// ============================================================================
// CefDownloadHandler methods
// ============================================================================

bool BrowserClient::CanDownload(CefRefPtr<CefBrowser> browser,
                                 const CefString& url,
                                 const CefString& request_method) {
    // Allow all downloads
    return true;
}

void BrowserClient::OnBeforeDownload(
    CefRefPtr<CefBrowser> browser,
    CefRefPtr<CefDownloadItem> download_item,
    const CefString& suggested_name,
    CefRefPtr<CefBeforeDownloadCallback> callback) {
    CEF_REQUIRE_UI_THREAD();

    // Continue download with default path and show save dialog
    callback->Continue("", true);
}

void BrowserClient::OnDownloadUpdated(
    CefRefPtr<CefBrowser> browser,
    CefRefPtr<CefDownloadItem> download_item,
    CefRefPtr<CefDownloadItemCallback> callback) {
    CEF_REQUIRE_UI_THREAD();

    if (download_item->IsComplete()) {
        // Download complete
    } else if (download_item->IsCanceled()) {
        // Download cancelled
    } else {
        // Download in progress
        // Could update UI with progress here
    }
}

// ============================================================================
// Utility methods
// ============================================================================

void BrowserClient::CloseAllBrowsers(bool force_close) {
    if (!CefCurrentlyOn(TID_UI)) {
        CefPostTask(TID_UI,
                    base::BindOnce(&BrowserClient::CloseAllBrowsers, this,
                                   force_close));
        return;
    }

    if (browser_list_.empty()) {
        return;
    }

    for (auto& browser : browser_list_) {
        browser->GetHost()->CloseBrowser(force_close);
    }
}
