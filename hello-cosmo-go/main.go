package main

import (
	"embed"
	"encoding/json"
	"mime"
	"strings"

	hello_cosmo "hello_cosmo/gen"
)

//go:embed ui
var embeddedUI embed.FS

const BUCKET string = ""

// TODO: move to kv module(?)
type MyHelloCosmo struct{}

func (kv *MyHelloCosmo) Handle(request hello_cosmo.WasiHttpIncomingHandlerIncomingRequest, response hello_cosmo.WasiHttpHttpTypesResponseOutparam) {
	hello_cosmo.WasiLoggingLoggingLog(hello_cosmo.WasiLoggingLoggingLevelInfo(), "go-component", "beginning Handle")

	method, pathWithQuery := methodAndPath(request)
	if pathWithQuery.IsNone() {
		return
	}

	splitPathQuery := strings.Split(pathWithQuery.Unwrap(), "?")

	path := splitPathQuery[0]
	trimmedPath := strings.Split(strings.TrimPrefix(path, "/"), "/")

	switch {
	case method == hello_cosmo.WasiHttpHttpTypesMethodGet() && len(trimmedPath) >= 2 && (trimmedPath[0] == "api" && trimmedPath[1] == "counter"):
		bucket := hello_cosmo.WasiKeyvalueTypesOpenBucket(BUCKET)
		if bucket.IsErr() {
			return
		}

		var newNum uint32
		if len(trimmedPath) == 3 && trimmedPath[2] != "" {
			newNum = IncrementCounter(bucket.Unwrap(), trimmedPath[2], 1)
		} else {
			newNum = IncrementCounter(bucket.Unwrap(), "default", 1)
		}

		resp := struct {
			Counter uint32 `json:"counter"`
		}{
			Counter: newNum,
		}

		bResp, err := json.Marshal(resp)
		if err != nil {
			return
		}

		writeHttpResponse(response, 200, contentTypeJsonHeaders(), bResp)
	default:
		if path == "/" {
			path = "ui/index.html"
		} else {
			path = "ui" + path
		}

		page, err := embeddedUI.ReadFile(path)
		if err != nil {
			writeHttpResponse(response, 404, contentTypeJsonHeaders(), []byte("{\"error\":\""+path+": not found\"}"))
		}

		ext := ""
		extSplit := strings.Split(path, ".")
		if len(extSplit) > 1 {
			ext = extSplit[len(extSplit)-1]
		}

		writeHttpResponse(response, 200, contentTypeMimeHeaders([]byte(mime.TypeByExtension(ext))), page)
	}

}

func init() {
	myhc := new(MyHelloCosmo)
	hello_cosmo.SetExportsWasiHttpIncomingHandler(myhc)
}

//go:generate wit-bindgen tiny-go ./wit -w hello-cosmo --out-dir=gen
func main() {}
