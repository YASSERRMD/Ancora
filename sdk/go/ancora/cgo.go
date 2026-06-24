package ancora

// #cgo CFLAGS: -I${SRCDIR}/../../../crates/ancora-ffi/include
// #cgo LDFLAGS: -L${SRCDIR}/../../../target/debug -lancora_ffi
// #cgo linux LDFLAGS: -lpthread -ldl
import "C"
