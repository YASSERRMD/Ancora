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

func asError(code C.AncorErrorCode) error {
	if code == C.Ok {
		return nil
	}
	return ErrorCode(code)
}

func (e ErrorCode) Error() string {
	switch e {
	case ErrOk:
		return "ok"
	case ErrNullPtr:
		return "null pointer"
	case ErrInvalidUTF8:
		return "invalid utf-8"
	case ErrInternal:
		return "internal error"
	default:
		return "unknown error"
	}
}
