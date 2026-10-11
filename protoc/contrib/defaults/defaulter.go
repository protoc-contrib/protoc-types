// Package defaults holds the protoc.contrib.defaults options read by
// protoc-gen-go-defaults, and the Defaulter interface its generated
// SetDefaults() methods satisfy.
package defaults

// Defaulter is implemented by every message that protoc-gen-go-defaults
// generates a SetDefaults() method for, and by the hand-written SetDefaults()
// of a (protoc.contrib.defaults.skip) or (protoc.contrib.defaults.unexported)
// message.
//
// It lets code that does not hold the concrete type, such as a gRPC
// interceptor, apply defaults without reflection:
//
//	if d, ok := req.(defaults.Defaulter); ok {
//		d.SetDefaults()
//	}
type Defaulter interface {
	// SetDefaults populates unset fields with the defaults declared in the
	// .proto source. Fields that are already set are preserved.
	SetDefaults()
}
