// CEF Browser - Application Handler
#ifndef CEF_BROWSER_APP_H_
#define CEF_BROWSER_APP_H_

#include "include/cef_app.h"
#include "include/cef_browser_process_handler.h"
#include "include/cef_render_process_handler.h"

// Application handler that manages browser and renderer processes
class BrowserApp : public CefApp, public CefBrowserProcessHandler, public CefRenderProcessHandler {
public:
    BrowserApp();

    // CefApp methods
    CefRefPtr<CefBrowserProcessHandler> GetBrowserProcessHandler() override { return this; }
    CefRefPtr<CefRenderProcessHandler> GetRenderProcessHandler() override { return this; }
    void OnBeforeCommandLineProcessing(const CefString& process_type,
                                       CefRefPtr<CefCommandLine> command_line) override;

    // CefBrowserProcessHandler methods
    void OnContextInitialized() override;
    CefRefPtr<CefClient> GetDefaultClient() override;

    // CefRenderProcessHandler methods
    void OnWebKitInitialized() override;
    void OnContextCreated(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame,
                          CefRefPtr<CefV8Context> context) override;

private:
    IMPLEMENT_REFCOUNTING(BrowserApp);
    DISALLOW_COPY_AND_ASSIGN(BrowserApp);
};

#endif  // CEF_BROWSER_APP_H_
