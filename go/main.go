package main

/*
typedef void (*callback_t)(void *, size_t, int *);

static void call_callback(void *state, size_t len, int *arr, callback_t cb) {
    cb(state, len, arr);
}
*/
import "C"
import "unsafe"

//export send_array
func send_array(state *C.void, callback C.callback_t) {
	arr := []int32{1, 2, 3}
	C.call_callback(unsafe.Pointer(state), C.size_t(len(arr)), (*C.int)(unsafe.Pointer(&arr[0])), callback)
}

func main() {}
