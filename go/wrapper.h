#include <stdint.h>
#include <stdlib.h>

typedef void *(*new_array_callback)(size_t len, void *data);
extern void *call_array_callback(size_t len, void *data, new_array_callback cb);
