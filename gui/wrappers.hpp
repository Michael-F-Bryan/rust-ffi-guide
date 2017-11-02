#include "client.h"
#include <string>
#include <vector>

class Response {
public:
  std::vector<char> read_body();
  Response(ffi::Response *raw) : raw(raw){};
  ~Response();

private:
  ffi::Response *raw;
};

class Request {
public:
  Request(const std::string &);
  Response send();
  ~Request();

private:
  ffi::Request *raw;
};
