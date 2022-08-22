package main

/*
struct Message {
	int id;
	char *text;
};
*/
import "C"
import "fmt"

//export greeting
func greeting(message *C.struct_Message) {
	fmt.Printf("%d: %s\n", message.id, C.GoString(message.text))
}

func main() {}
