#include <string>
#include <vector>

class Response {
public:
  std::vector<char> read_body();
  Response(void *raw) : raw(raw){};
  ~Response();

private:
  void *raw;
};

class Request {
public:
  Request(const std::string);
  Response send();
  ~Request();

private:
  void *raw;
};
