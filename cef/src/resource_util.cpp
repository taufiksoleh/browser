// CEF Browser - Resource Utilities Implementation
#include "resource_util.h"

#include <fstream>
#include <sstream>

#include "include/cef_parser.h"
#include "include/wrapper/cef_helpers.h"

#if defined(OS_WIN)
#include <shlobj.h>
#include <windows.h>
#elif defined(OS_LINUX) || defined(OS_MAC)
#include <pwd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#endif

std::string GetDataURI(const std::string& data, const std::string& mime_type) {
    return "data:" + mime_type + ";base64," +
           CefURIEncode(CefBase64Encode(data.data(), data.size()), false).ToString();
}

bool LoadBinaryResource(const char* resource_name, std::string& resource_data) {
    std::string path = GetResourcesDir() + "/" + resource_name;

    std::ifstream file(path, std::ios::binary | std::ios::ate);
    if (!file.is_open()) {
        return false;
    }

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);

    resource_data.resize(static_cast<size_t>(size));
    if (!file.read(&resource_data[0], size)) {
        return false;
    }

    return true;
}

std::string GetApplicationDir() {
    std::string result;

#if defined(OS_WIN)
    char path[MAX_PATH];
    if (GetModuleFileNameA(nullptr, path, MAX_PATH) > 0) {
        result = path;
        size_t pos = result.find_last_of("\\/");
        if (pos != std::string::npos) {
            result = result.substr(0, pos);
        }
    }
#elif defined(OS_LINUX)
    char path[4096];
    ssize_t count = readlink("/proc/self/exe", path, sizeof(path) - 1);
    if (count != -1) {
        path[count] = '\0';
        result = path;
        size_t pos = result.find_last_of('/');
        if (pos != std::string::npos) {
            result = result.substr(0, pos);
        }
    }
#elif defined(OS_MAC)
    char path[4096];
    uint32_t size = sizeof(path);
    if (_NSGetExecutablePath(path, &size) == 0) {
        result = path;
        size_t pos = result.find_last_of('/');
        if (pos != std::string::npos) {
            result = result.substr(0, pos);
        }
    }
#endif

    return result;
}

std::string GetResourcesDir() {
    std::string app_dir = GetApplicationDir();

#if defined(OS_MAC)
    // On macOS, resources are in the app bundle
    return app_dir + "/../Resources";
#else
    // On other platforms, resources are alongside the executable
    return app_dir + "/resources";
#endif
}

std::string GetUserDataDir() {
    std::string result;

#if defined(OS_WIN)
    char path[MAX_PATH];
    if (SHGetFolderPathA(nullptr, CSIDL_LOCAL_APPDATA, nullptr, 0, path) == S_OK) {
        result = std::string(path) + "\\CEFBrowser";
    }
#elif defined(OS_LINUX) || defined(OS_MAC)
    const char* home = getenv("HOME");
    if (!home) {
        struct passwd* pwd = getpwuid(getuid());
        if (pwd) {
            home = pwd->pw_dir;
        }
    }
    if (home) {
#if defined(OS_MAC)
        result = std::string(home) + "/Library/Application Support/CEFBrowser";
#else
        result = std::string(home) + "/.config/cef-browser";
#endif
    }
#endif

    // Create directory if it doesn't exist
    CreateDirectory(result);

    return result;
}

bool FileExists(const std::string& path) {
#if defined(OS_WIN)
    DWORD attrs = GetFileAttributesA(path.c_str());
    return (attrs != INVALID_FILE_ATTRIBUTES);
#else
    struct stat st;
    return stat(path.c_str(), &st) == 0;
#endif
}

bool CreateDirectory(const std::string& path) {
    if (path.empty()) {
        return false;
    }

#if defined(OS_WIN)
    return CreateDirectoryA(path.c_str(), nullptr) || GetLastError() == ERROR_ALREADY_EXISTS;
#else
    return mkdir(path.c_str(), 0755) == 0 || errno == EEXIST;
#endif
}
