package ancora

// AncorError is the Go mapping of the C AncorErrorCode FFI enum.
type AncorError int32

const (
	ErrOk          AncorError = 0
	ErrNullPtr     AncorError = 1
	ErrInvalidUTF8 AncorError = 2
	ErrInternal    AncorError = 3
)

func asError(code uint32) error {
	if code == 0 {
		return nil
	}
	return AncorError(code)
}

func (e AncorError) Error() string {
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
