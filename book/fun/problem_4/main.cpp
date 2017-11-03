#include <iostream>
#include <string>

extern "C" {
void log_message(int, const char *);
}

int main() {
  std::string message = "Hello World";
  log_message(0x04 | 0x01, message.c_str());
}