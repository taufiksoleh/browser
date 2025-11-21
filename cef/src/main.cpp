// CEF Browser - Main Entry Point
// A production-ready web browser using Chromium Embedded Framework

#include "include/cef_app.h"
#include "include/cef_browser.h"
#include "include/cef_command_line.h"

#include "app.h"
#include "browser_window.h"

#if defined(OS_WIN)
#include <windows.h>
#endif

#if defined(OS_LINUX)
#include <X11/Xlib.h>
#endif

namespace {

// Returns the main application entry point.
int RunMain(int argc, char* argv[]) {
#if defined(OS_LINUX)
    // Initialize X11 threading support
    XInitThreads();
#endif

    // Create main args
    CefMainArgs main_args(argc, argv);

    // Create the application instance
    CefRefPtr<BrowserApp> app(new BrowserApp);

    // Execute the subprocess if this is a renderer/gpu/etc process
    int exit_code = CefExecuteProcess(main_args, app, nullptr);
    if (exit_code >= 0) {
        // The sub-process has completed
        return exit_code;
    }

    // Parse command line arguments
    CefRefPtr<CefCommandLine> command_line = CefCommandLine::CreateCommandLine();
    command_line->InitFromArgv(argc, argv);

    // Configure CEF settings
    CefSettings settings;

    // Enable GPU acceleration
    settings.windowless_rendering_enabled = false;

    // Use hardware acceleration
    settings.chrome_runtime = true;

    // Set cache path
    CefString(&settings.cache_path).FromASCII("./cache");

    // Set user data path
    CefString(&settings.user_data_path).FromASCII("./user_data");

    // Set log file
    CefString(&settings.log_file).FromASCII("./cef_debug.log");
    settings.log_severity = LOGSEVERITY_INFO;

    // Remote debugging (optional - useful for development)
    settings.remote_debugging_port = 9222;

    // Locale
    CefString(&settings.locale).FromASCII("en-US");

    // Background color (white)
    settings.background_color = CefColorSetARGB(255, 255, 255, 255);

    // Multi-threaded message loop
    settings.multi_threaded_message_loop = false;
    settings.external_message_pump = false;

    // Initialize CEF
    if (!CefInitialize(main_args, settings, app, nullptr)) {
        return 1;
    }

    // Create the browser window
    BrowserWindow::Create();

    // Run the CEF message loop
    CefRunMessageLoop();

    // Shutdown CEF
    CefShutdown();

    return 0;
}

}  // namespace

#if defined(OS_WIN)
int WINAPI WinMain(HINSTANCE hInstance,
                   HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine,
                   int nCmdShow) {
    return RunMain(__argc, __argv);
}
#else
int main(int argc, char* argv[]) {
    return RunMain(argc, argv);
}
#endif
