// CEF Browser - Browser Window Manager
#ifndef CEF_BROWSER_WINDOW_H_
#define CEF_BROWSER_WINDOW_H_

#include <string>
#include "include/cef_browser.h"

// Browser window manager - creates and manages the main browser window
class BrowserWindow {
public:
    // Create the main browser window
    static void Create();

    // Navigate to a URL
    static void Navigate(const std::string& url);

    // Navigate back
    static void GoBack();

    // Navigate forward
    static void GoForward();

    // Reload the current page
    static void Reload();

    // Stop loading
    static void StopLoading();

    // Get the main browser instance
    static CefRefPtr<CefBrowser> GetBrowser();

    // Default window dimensions
    static const int kDefaultWidth = 1280;
    static const int kDefaultHeight = 800;

    // Default home page
    static const char* kDefaultUrl;

private:
    BrowserWindow() = delete;
};

#endif  // CEF_BROWSER_WINDOW_H_
