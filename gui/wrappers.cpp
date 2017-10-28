#include "wrappers.hpp"
#include <cassert>
#include <string>
#include <vector>

extern "C" {
void *request_create(const char *);
void request_destroy(void *);
void response_destroy(void *);
int response_body_length(void *);
int response_body(void *, char *, int);
void *request_send(void *);
}

Request::~Request() { request_destroy(raw); }

Request::Request(const std::string &url) {
  raw = request_create(url.c_str());
  if (raw == nullptr) {
    throw "Invalid URL";
  }
}

Response Request::send() {
  void *raw_response = request_send(raw);

  if (raw_response == nullptr) {
    throw "Request failed";
  }

  return Response(raw_response);
}

Response::~Response() { response_destroy(raw); }

std::vector<char> Response::read_body() {
  int length = response_body_length(raw);
  assert(length >= 0);

  std::vector<char> buffer(length);

  int bytes_written = response_body(raw, buffer.data(), buffer.size());
  assert(bytes_written == length);

  return buffer;
}
