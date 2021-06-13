#include <cstdint>
#include <iostream>

extern "C" {
uint32_t add(uint32_t, uint32_t);
}

int main() {
  uint32_t a = 5, b = 10;

  uint32_t sum = add(a, b);
  std::cout << "The sum of " << a << " and " << b << " is " << sum << std::endl;
}
