#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#define ERROR_BUFFER_TOO_SMALL 5

#define ERROR_GENERAL 0

#define ERROR_NOT_FOUND 2

#define ERROR_PARSE 4

#define ERROR_PERMISSION_DENIED 3

#define ERROR_UTF8 1

typedef struct Value Value;

/*
 * Extra information about an error.
 */
typedef struct {
  /*
   * A human-friendly error message (`null` if there wasn't one).
   */
  const char *msg;
  /*
   * The general error category.
   */
  uint32_t category;
} Error;

/*
 * Get a short description of an error's category.
 */
const char *category_name(uint32_t category);

/*
 * Clear the `LAST_ERROR` variable.
 */
void clear_error(void);

/*
 * Retrieve the most recent `Error` from the `LAST_ERROR` variable.
 *
 * # Safety
 *
 * The error message will be freed if another error occurs. It is the caller's
 * responsibility to make sure they're no longer using the `Error` before
 * calling any function which may set `LAST_ERROR`.
 */
Error last_error(void);

/*
 * Parse a TOML file into a `Value`.
 *
 * # Note
 *
 * Don't forget to free the `Value` once you're done with it using
 * `value_destroy()`.
 */
const Value *parse_file(const char *filename);

int32_t value_as_str(const Value *value, char *buffer, int32_t len);

/*
 * Destroy a `Value` once it's no longer needed.
 */
void value_destroy(const Value *value);

/*
 * Get the `element` item out of a `Value`.
 */
const Value *value_get(const Value *value, const char *element);
