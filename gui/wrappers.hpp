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
  ~Request();

private:
  ffi::Request *raw;
};

class PluginManager {
public:
  PluginManager();
  ~PluginManager();
  void unload();
  void pre_send(Request& req);
  void post_receive(Response& res);

private:
  ffi::PluginManager *raw;
};