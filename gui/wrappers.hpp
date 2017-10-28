#include <string>

class Request {
public:
  Request(const std::string);
  ~Request();

private:
  void *raw;
};
