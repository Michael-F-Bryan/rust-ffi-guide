#include "wrappers.hpp"
#include "client.hpp"
#include <string>
#include <vector>

Request::~Request() { ffi::request_destroy(raw); }

Request::Request(const std::string &url) {
  raw = ffi::request_create(url.c_str());
  if (raw == nullptr) {
    throw WrapperException::last_error();
  }
}

Response Request::send() {
  ffi::Response *raw_response = ffi::request_send(raw);

  if (raw_response == nullptr) {
    throw WrapperException::last_error();
  }

  return Response(raw_response);
}

Response::~Response() { ffi::response_destroy(raw); }

std::vector<char> Response::read_body() {
  int length = ffi::response_body_length(raw);
  if (length < 0) {
    throw WrapperException::last_error();
  }

  std::vector<char> buffer(length);

  int bytes_written = ffi::response_body(raw, buffer.data(), buffer.size());
  if (bytes_written != length) {
    throw WrapperException::last_error();
  }

  return buffer;
}

PluginManager::PluginManager() { raw = ffi::plugin_manager_new(); }

PluginManager::~PluginManager() { ffi::plugin_manager_destroy(raw); }

void PluginManager::pre_send(Request &req) {
  ffi::plugin_manager_pre_send(raw, req.raw);
}

void PluginManager::unload() { ffi::plugin_manager_unload(raw); }

void PluginManager::post_receive(Response &res) {
  ffi::plugin_manager_post_receive(raw, res.raw);
}

void PluginManager::load_plugin(const std::string& filename) {
  int ret = ffi::plugin_manager_load_plugin(raw, filename.c_str());

  if (ret != 0) {
    throw WrapperException::last_error();
  }
}

std::string last_error_message() {
  int error_length = ffi::last_error_length();

  if (error_length == 0) {
    return std::string();
  }

  std::string msg(error_length, '\0');
  int ret = ffi::last_error_message(&msg[0], msg.length());
  if (ret <= 0) {
    // If we ever get here it's a bug
    throw WrapperException("Fetching error message failed");
  }

  return msg;
}

WrapperException WrapperException::last_error() {
  std::string msg = last_error_message();

  if (msg.length() == 0) {
    return WrapperException("(no error available)");
  } else {
    return WrapperException(msg);
  }
}