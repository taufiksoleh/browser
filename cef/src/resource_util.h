// CEF Browser - Resource Utilities
#ifndef CEF_BROWSER_RESOURCE_UTIL_H_
#define CEF_BROWSER_RESOURCE_UTIL_H_

#include <string>
#include "include/cef_base.h"

// Get the data URI for HTML content
std::string GetDataURI(const std::string& data, const std::string& mime_type);

// Load a resource from the filesystem
bool LoadBinaryResource(const char* resource_name, std::string& resource_data);

// Get the application directory path
std::string GetApplicationDir();

// Get the resources directory path
std::string GetResourcesDir();

// Get the user data directory path
std::string GetUserDataDir();

// Check if a file exists
bool FileExists(const std::string& path);

// Create directory if it doesn't exist
bool CreateDirectory(const std::string& path);

#endif  // CEF_BROWSER_RESOURCE_UTIL_H_
