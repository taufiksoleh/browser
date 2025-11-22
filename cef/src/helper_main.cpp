// CEF Browser - Helper Process Entry Point
// This executable handles renderer, GPU, and other CEF subprocesses

#include "app.h"
#include "include/cef_app.h"

#if defined(OS_WIN)
#include <windows.h>
#endif

#if defined(OS_WIN)
int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    CefMainArgs main_args(hInstance);
#else
int main(int argc, char* argv[]) {
    CefMainArgs main_args(argc, argv);
#endif

    // Create the application instance
    CefRefPtr<BrowserApp> app(new BrowserApp);

    // Execute the subprocess
    return CefExecuteProcess(main_args, app, nullptr);
}
