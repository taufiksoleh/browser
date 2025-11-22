// CEF Browser - Browser Client Handler
#ifndef CEF_BROWSER_CLIENT_H_
#define CEF_BROWSER_CLIENT_H_

#include <list>
#include <string>

#include "include/cef_client.h"
#include "include/cef_context_menu_handler.h"
#include "include/cef_display_handler.h"
#include "include/cef_download_handler.h"
#include "include/cef_keyboard_handler.h"
#include "include/cef_life_span_handler.h"
#include "include/cef_load_handler.h"
#include "include/cef_request_handler.h"

// Browser client that handles browser events and callbacks
class BrowserClient : public CefClient,
                      public CefLifeSpanHandler,
                      public CefDisplayHandler,
                      public CefLoadHandler,
                      public CefRequestHandler,
                      public CefContextMenuHandler,
                      public CefKeyboardHandler,
                      public CefDownloadHandler {
public:
    explicit BrowserClient();
    ~BrowserClient() override;

    // CefClient methods
    CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override { return this; }
    CefRefPtr<CefDisplayHandler> GetDisplayHandler() override { return this; }
    CefRefPtr<CefLoadHandler> GetLoadHandler() override { return this; }
    CefRefPtr<CefRequestHandler> GetRequestHandler() override { return this; }
    CefRefPtr<CefContextMenuHandler> GetContextMenuHandler() override { return this; }
    CefRefPtr<CefKeyboardHandler> GetKeyboardHandler() override { return this; }
    CefRefPtr<CefDownloadHandler> GetDownloadHandler() override { return this; }

    // CefLifeSpanHandler methods
    bool OnBeforePopup(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                       const CefString& target_url, const CefString& target_frame_name,
                       CefLifeSpanHandler::WindowOpenDisposition target_disposition,
                       bool user_gesture, const CefPopupFeatures& popupFeatures,
                       CefWindowInfo& windowInfo, CefRefPtr<CefClient>& client,
                       CefBrowserSettings& settings, CefRefPtr<CefDictionaryValue>& extra_info,
                       bool* no_javascript_access) override;
    void OnAfterCreated(CefRefPtr<CefBrowser> browser) override;
    bool DoClose(CefRefPtr<CefBrowser> browser) override;
    void OnBeforeClose(CefRefPtr<CefBrowser> browser) override;

    // CefDisplayHandler methods
    void OnTitleChange(CefRefPtr<CefBrowser> browser, const CefString& title) override;
    void OnAddressChange(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                         const CefString& url) override;
    void OnFullscreenModeChange(CefRefPtr<CefBrowser> browser, bool fullscreen) override;
    bool OnConsoleMessage(CefRefPtr<CefBrowser> browser, cef_log_severity_t level,
                          const CefString& message, const CefString& source, int line) override;

    // CefLoadHandler methods
    void OnLoadingStateChange(CefRefPtr<CefBrowser> browser, bool isLoading, bool canGoBack,
                              bool canGoForward) override;
    void OnLoadStart(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                     TransitionType transition_type) override;
    void OnLoadEnd(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                   int httpStatusCode) override;
    void OnLoadError(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, ErrorCode errorCode,
                     const CefString& errorText, const CefString& failedUrl) override;

    // CefRequestHandler methods
    bool OnBeforeBrowse(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                        CefRefPtr<CefRequest> request, bool user_gesture,
                        bool is_redirect) override;

    // CefContextMenuHandler methods
    void OnBeforeContextMenu(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                             CefRefPtr<CefContextMenuParams> params,
                             CefRefPtr<CefMenuModel> model) override;
    bool OnContextMenuCommand(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                              CefRefPtr<CefContextMenuParams> params, int command_id,
                              EventFlags event_flags) override;

    // CefKeyboardHandler methods
    bool OnPreKeyEvent(CefRefPtr<CefBrowser> browser, const CefKeyEvent& event,
                       CefEventHandle os_event, bool* is_keyboard_shortcut) override;
    bool OnKeyEvent(CefRefPtr<CefBrowser> browser, const CefKeyEvent& event,
                    CefEventHandle os_event) override;

    // CefDownloadHandler methods
    bool CanDownload(CefRefPtr<CefBrowser> browser, const CefString& url,
                     const CefString& request_method) override;
    void OnBeforeDownload(CefRefPtr<CefBrowser> browser, CefRefPtr<CefDownloadItem> download_item,
                          const CefString& suggested_name,
                          CefRefPtr<CefBeforeDownloadCallback> callback) override;
    void OnDownloadUpdated(CefRefPtr<CefBrowser> browser, CefRefPtr<CefDownloadItem> download_item,
                           CefRefPtr<CefDownloadItemCallback> callback) override;

    // Browser access
    CefRefPtr<CefBrowser> GetBrowser() const { return browser_; }

    // Check if browser is closing
    bool IsClosing() const { return is_closing_; }

    // Close all browsers
    void CloseAllBrowsers(bool force_close);

    // Get browser count
    static int GetBrowserCount() { return browser_count_; }

private:
    CefRefPtr<CefBrowser> browser_;
    std::list<CefRefPtr<CefBrowser>> browser_list_;
    bool is_closing_;
    static int browser_count_;

    IMPLEMENT_REFCOUNTING(BrowserClient);
    DISALLOW_COPY_AND_ASSIGN(BrowserClient);
};

#endif  // CEF_BROWSER_CLIENT_H_
