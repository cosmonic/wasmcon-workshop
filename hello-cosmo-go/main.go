package main

import (
	"strings"

	hello_cosmo "hello_cosmo/gen"
)

type MyHelloCosmo struct{}

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
