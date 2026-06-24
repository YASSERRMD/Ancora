package ancora

// #include "ancora.h"
import "C"

// Runtime is an opaque handle to an Ancora runtime instance.
type Runtime struct {
	ptr *C.AncorRuntime
}
