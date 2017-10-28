#include "wrappers.hpp"
#include <string>

extern "C" {
void *request_create(const char *);
void request_destroy(void *);
}

Request::~Request() { request_destroy(raw); }

Request::Request(const std::string url) {
  raw = request_create(url.c_str());
  if (raw == nullptr) {
    throw "Invalid URL";
  }
}