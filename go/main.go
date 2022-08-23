package main

/*
#include "wrapper.h"
*/
import "C"
import "unsafe"

//export send_array
func send_array(cb C.new_array_callback) *C.void {
	arr := []int32{1, 2, 3}
	ret := C.call_array_callback(C.size_t(len(arr)), unsafe.Pointer(&arr[0]), cb)
	return (*C.void)(ret)
}

func main() {}
