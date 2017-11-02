#include "wrappers.hpp"
#include "client.h"
#include <cassert>
#include <string>
#include <vector>

Request::~Request() { ffi::request_destroy(raw); }

Request::Request(const std::string &url) {
  raw = ffi::request_create(url.c_str());
  if (raw == nullptr) {
    throw "Invalid URL";
  }
}

Response Request::send() {
  ffi::Response *raw_response = ffi::request_send(raw);

  if (raw_response == nullptr) {
    throw "Request failed";
  }

  return Response(raw_response);
}

Response::~Response() { ffi::response_destroy(raw); }

std::vector<char> Response::read_body() {
  int length = ffi::response_body_length(raw);
  assert(length >= 0);

  std::vector<char> buffer(length);

  int bytes_written = ffi::response_body(raw, buffer.data(), buffer.size());
  assert(bytes_written == length);

  return buffer;
}
