//go:generate protoc --proto_path=../../crates/ancora-proto/proto --go_out=. --go_opt=module=github.com/YASSERRMD/Ancora/bindings/go ../../crates/ancora-proto/proto/messages.proto ../../crates/ancora-proto/proto/contracts.proto ../../crates/ancora-proto/proto/journal.proto ../../crates/ancora-proto/proto/errors.proto
//go:generate protoc --proto_path=../../crates/ancora-grpc/proto --go_out=ancora/grpc/ --go_opt=paths=source_relative --go-grpc_out=ancora/grpc/ --go-grpc_opt=paths=source_relative ../../crates/ancora-grpc/proto/ancora.proto

package sdk
