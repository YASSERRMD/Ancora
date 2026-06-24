package ancora

// #include "ancora.h"
import "C"

// ErrorCode maps to the C AncorErrorCode enum.
type ErrorCode int32

const (
	ErrOk          ErrorCode = 0
	ErrNullPtr     ErrorCode = 1
	ErrInvalidUTF8 ErrorCode = 2
	ErrInternal    ErrorCode = 3
)
