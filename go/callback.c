#include "wrapper.h"

void *call_array_callback(size_t len, void *data, new_array_callback cb) {
  return cb(len, data);
}
