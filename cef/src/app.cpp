// CEF Browser - Application Handler Implementation
#include "app.h"

#include "include/cef_browser.h"
#include "include/cef_command_line.h"
#include "include/wrapper/cef_helpers.h"

BrowserApp::BrowserApp() {}

void BrowserApp::OnBeforeCommandLineProcessing(
    const CefString& process_type,
    CefRefPtr<CefCommandLine> command_line) {
    // Add command line switches for better performance and compatibility

    // Enable hardware acceleration
    command_line->AppendSwitch("enable-gpu");
    command_line->AppendSwitch("enable-gpu-rasterization");
    command_line->AppendSwitch("enable-zero-copy");

    // Enable smooth scrolling
    command_line->AppendSwitch("enable-smooth-scrolling");

    // Enable experimental web features
    command_line->AppendSwitch("enable-experimental-web-platform-features");

    // Disable some security features for local development (remove in production)
    // command_line->AppendSwitch("disable-web-security");

    // Enable remote debugging
    command_line->AppendSwitchWithValue("remote-debugging-port", "9222");

    // GPU process settings
    command_line->AppendSwitch("ignore-gpu-blocklist");

    // Renderer process limit (0 = unlimited)
    command_line->AppendSwitchWithValue("renderer-process-limit", "4");

    // Enable tab discarding when memory is low
    command_line->AppendSwitch("enable-tab-discarding");
}

void BrowserApp::OnContextInitialized() {
    CEF_REQUIRE_UI_THREAD();

    // Browser process has been initialized
    // The browser window will be created from main.cpp
}

CefRefPtr<CefClient> BrowserApp::GetDefaultClient() {
    // Return nullptr - browser client is created in BrowserWindow::Create()
    return nullptr;
}

void BrowserApp::OnWebKitInitialized() {
    // Called in the renderer process when WebKit has been initialized
    // This is where you can register custom JavaScript bindings
}

void BrowserApp::OnContextCreated(CefRefPtr<CefBrowser> browser,
                                   CefRefPtr<CefFrame> frame,
                                   CefRefPtr<CefV8Context> context) {
    // Called when a new V8 context is created
    // You can inject custom JavaScript objects here

    // Example: Inject a custom 'browser' object
    CefRefPtr<CefV8Value> global = context->GetGlobal();

    // Create browser API object
    CefRefPtr<CefV8Value> browserObj = CefV8Value::CreateObject(nullptr, nullptr);

    // Add version property
    browserObj->SetValue("version",
                         CefV8Value::CreateString("CEF Browser 1.0.0"),
                         V8_PROPERTY_ATTRIBUTE_READONLY);

    // Add platform property
#if defined(OS_WIN)
    browserObj->SetValue("platform",
                         CefV8Value::CreateString("windows"),
                         V8_PROPERTY_ATTRIBUTE_READONLY);
#elif defined(OS_LINUX)
    browserObj->SetValue("platform",
                         CefV8Value::CreateString("linux"),
                         V8_PROPERTY_ATTRIBUTE_READONLY);
#elif defined(OS_MAC)
    browserObj->SetValue("platform",
                         CefV8Value::CreateString("macos"),
                         V8_PROPERTY_ATTRIBUTE_READONLY);
#endif

    // Register the browser object globally
    global->SetValue("cefBrowser", browserObj, V8_PROPERTY_ATTRIBUTE_NONE);
}
