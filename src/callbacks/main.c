#include <stddef.h>
#include <assert.h>

char get_item_10000(char *buffer, size_t len);
char safe_get_item_10000(char *buffer, size_t len);

int main() {
  char buffer[50] = {};
  char got = safe_get_item_10000(buffer, 50);

  assert(got == 0);
}
