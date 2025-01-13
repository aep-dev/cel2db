package main

// TODO: in the final packaging process, we need to have the rust compiled library
// in this directory and have LDFLAGS to point to it. This will make the library
// consumable via `go get`.

// extern char *cel_to_sql(const char *cel_expr, const char *sql_dialect);
import "C"
import "fmt"

func main() {
	sql := C.cel_to_sql(C.CString("1 + 2"), C.CString("sqlite"))
	fmt.Println(C.GoString(sql))
}
