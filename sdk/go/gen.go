//go:generate protoc --proto_path=../../crates/ancora-proto/proto --go_out=. --go_opt=module=github.com/YASSERRMD/Ancora/bindings/go ../../crates/ancora-proto/proto/messages.proto ../../crates/ancora-proto/proto/contracts.proto ../../crates/ancora-proto/proto/journal.proto ../../crates/ancora-proto/proto/errors.proto

package sdk
