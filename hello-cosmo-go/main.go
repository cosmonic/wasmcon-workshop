package main

import (
	"fmt"
	"strings"

	hello_cosmo "hello_cosmo/gen"
)

type MyHelloCosmo struct{}

func increment(key string) (uint32, error) {
	bucket := hello_cosmo.WasiKeyvalueTypesOpenBucket("")
	if bucket.IsErr() {
		return 0, fmt.Errorf("error opening bucket")
	}

	var newNum uint32 = IncrementCounter(bucket.Unwrap(), key, 1)

	hello_cosmo.WasiLoggingLoggingLog(hello_cosmo.WasiLoggingLoggingLevelInfo(), "go-component", fmt.Sprintf("new value: %d", newNum))

	return newNum, nil
}

func (kv *MyHelloCosmo) Handle(request hello_cosmo.WasiHttpIncomingHandlerIncomingRequest, response hello_cosmo.WasiHttpHttpTypesResponseOutparam) {
	hello_cosmo.WasiLoggingLoggingLog(hello_cosmo.WasiLoggingLoggingLevelInfo(), "go-component", "beginning Handle")

	method, pathWithQuery := methodAndPath(request)
	if pathWithQuery.IsNone() {
		return
	}

	splitPathQuery := strings.Split(pathWithQuery.Unwrap(), "?")

	path := splitPathQuery[0]
	hello_cosmo.WasiLoggingLoggingLog(hello_cosmo.WasiLoggingLoggingLevelInfo(), "go-component", path)

	switch {
	case method == hello_cosmo.WasiHttpHttpTypesMethodGet() && path == "/":
		writeHttpResponse(response, 200, contentTypeJsonHeaders(), []byte(`{"hello": "cosmo"}`))
	case method == hello_cosmo.WasiHttpHttpTypesMethodGet() && path == "/api/counter":
		newValue, err := increment("default")
		if err != nil {
			writeHttpResponse(response, 500, contentTypeJsonHeaders(), []byte(`{"error": "internal server error"}`))
		} else {
			writeHttpResponse(response, 200, contentTypeJsonHeaders(), []byte(fmt.Sprintf(`{"counter": %d}`, newValue)))
		}
	case method == hello_cosmo.WasiHttpHttpTypesMethodGet() && strings.HasPrefix(path, "/api/counter") && len(strings.Split(path, "/")) == 4:
		key := strings.Split(path, "/")[3]
		newValue, err := increment(key)
		if err != nil {
			writeHttpResponse(response, 500, contentTypeJsonHeaders(), []byte(`{"error": "internal server error"}`))
		} else {
			writeHttpResponse(response, 200, contentTypeJsonHeaders(), []byte(fmt.Sprintf(`{"counter": %d}`, newValue)))
		}
	default:
		writeHttpResponse(response, 404, contentTypeJsonHeaders(), []byte(`{"error": "not found"}`))
	}
}

func init() {
	myhc := new(MyHelloCosmo)
	hello_cosmo.SetExportsWasiHttpIncomingHandler(myhc)
}

//go:generate wit-bindgen tiny-go ./wit -w hello-cosmo --out-dir=gen
func main() {}
