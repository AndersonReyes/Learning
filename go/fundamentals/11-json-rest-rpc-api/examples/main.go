// Command main demonstrates encoding/json concepts used in this topic's
// exercise: struct tags with omitempty, json.RawMessage for
// shape-depends-on-a-field envelopes, json.Valid, and
// json.NewDecoder/DisallowUnknownFields for streaming decode — applied to
// small examples that are deliberately *not* the exercise (Store/ServeHTTP/
// HandleRPC in exercise.go).
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"strings"
)

// item demonstrates struct tags: Price is omitted from the JSON when zero.
type item struct {
	Name  string `json:"name"`
	Price int    `json:"price,omitempty"`
}

// envelope demonstrates json.RawMessage: Data's shape depends on Kind.
type envelope struct {
	Kind string          `json:"kind"`
	Data json.RawMessage `json:"data"`
}

func main() {
	// Struct tags + omitempty.
	data, _ := json.Marshal(item{Name: "widget"})
	fmt.Println("zero price omitted:", string(data))

	data, _ = json.Marshal(item{Name: "widget", Price: 5})
	fmt.Println("non-zero price included:", string(data))

	// json.RawMessage: decode the envelope, then decode Data once Kind is
	// known.
	raw := `{"kind":"item","data":{"name":"gadget","price":10}}`
	var env envelope
	if err := json.Unmarshal([]byte(raw), &env); err != nil {
		fmt.Println("unmarshal envelope error:", err)
		return
	}
	fmt.Println("envelope kind:", env.Kind)

	var decoded item
	if err := json.Unmarshal(env.Data, &decoded); err != nil {
		fmt.Println("unmarshal data error:", err)
		return
	}
	fmt.Printf("decoded item: %+v\n", decoded)

	// json.Valid.
	fmt.Println("valid JSON:", json.Valid([]byte(`{"a":1}`)))
	fmt.Println("invalid JSON:", json.Valid([]byte(`not json`)))

	// json.NewDecoder + DisallowUnknownFields: streaming decode that
	// rejects unexpected fields.
	dec := json.NewDecoder(strings.NewReader(`{"name":"widget","color":"red"}`))
	dec.DisallowUnknownFields()
	var strict item
	if err := dec.Decode(&strict); err != nil {
		fmt.Println("strict decode error:", err)
	}

	// json.NewEncoder: streaming encode to an io.Writer.
	var buf bytes.Buffer
	if err := json.NewEncoder(&buf).Encode(item{Name: "widget", Price: 5}); err != nil {
		fmt.Println("encode error:", err)
		return
	}
	fmt.Print("encoded with trailing newline: ", buf.String())
}
