// Package contrib_test guards the extension numbers of every protoc.contrib
// package. They all extend the same google.protobuf.*Options messages, so two
// packages claiming one number on one extendee would collide in any schema
// importing both.
package contrib_test

import (
	"strings"
	"testing"

	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/reflect/protoregistry"

	_ "github.com/protoc-contrib/protoc-types/protoc/contrib/defaults"
	_ "github.com/protoc-contrib/protoc-types/protoc/contrib/json"
)

func TestExtensionNumbersAreUnique(t *testing.T) {
	type key struct {
		extendee protoreflect.FullName
		number   protoreflect.FieldNumber
	}
	seen := map[key]protoreflect.FullName{}
	protoregistry.GlobalTypes.RangeExtensions(func(xt protoreflect.ExtensionType) bool {
		xd := xt.TypeDescriptor()
		if !strings.HasPrefix(string(xd.FullName()), "protoc.contrib.") {
			return true
		}
		k := key{xd.ContainingMessage().FullName(), xd.Number()}
		if other, ok := seen[k]; ok {
			t.Errorf("%s and %s both extend %s with number %d", other, xd.FullName(), k.extendee, k.number)
		}
		seen[k] = xd.FullName()
		return true
	})
	if len(seen) == 0 {
		t.Fatal("no protoc.contrib extensions registered")
	}
}

func TestExtensionNumbers(t *testing.T) {
	want := map[protoreflect.FullName]protoreflect.FieldNumber{
		"protoc.contrib.defaults.value":      55001,
		"protoc.contrib.defaults.oneof":      55002,
		"protoc.contrib.defaults.skip":       55003,
		"protoc.contrib.defaults.unexported": 55004,
		"protoc.contrib.json.skip":           55100,
		"protoc.contrib.json.enum_skip":      55100,
	}
	for name, number := range want {
		xt, err := protoregistry.GlobalTypes.FindExtensionByName(name)
		if err != nil {
			t.Errorf("%s: %v", name, err)
			continue
		}
		if got := xt.TypeDescriptor().Number(); got != number {
			t.Errorf("%s is numbered %d, want %d", name, got, number)
		}
	}
}
