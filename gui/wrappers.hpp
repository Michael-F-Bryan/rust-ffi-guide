#pragma once

#include "client.hpp"
#include <exception>
#include <string>
#include <vector>

class PluginManager;

class WrapperException : std::exception {
public:
  WrapperException(std::string msg) : msg(msg){};
  const std::string &message() { return msg; };

private:
  std::string msg;
};

class Response {
  friend class PluginManager;
  friend class Request;

public:
  std::vector<char> read_body();
  Response(const Response&) = delete;
  Response(Response&& other) {
    this->raw = other.raw;
    other.raw = nullptr;
  }
  ~Response();

private:
  Response(ffi::Response *raw) : raw(raw){};
  ffi::Response *raw;
};

class Request {
  friend class PluginManager;

public:
  Request(const std::string &);
  Response send();
  Request(const Request&) = delete;
  Request(Request&& other) {
    this->raw = other.raw;
    other.raw = nullptr;
  }
  ~Request();

private:
  ffi::Request *raw;
};

class PluginManager {
public:
  PluginManager();
  ~PluginManager();
  void unload();
  void load_plugin(const std::string& filename);
  void pre_send(Request &req);
  void post_receive(Response &res);

  PluginManager(const PluginManager&) = delete;
  PluginManager(PluginManager&& other) {
    this->raw = other.raw;
    other.raw = nullptr;
  }

private:
  ffi::PluginManager *raw;
};