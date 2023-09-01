package main

import (
	hello_cosmo "hello_cosmo/gen"
)

func writeHttpResponse(responseOutparam hello_cosmo.WasiHttpHttpTypesResponseOutparam, statusCode uint16, inHeaders []hello_cosmo.WasiHttpHttpTypesTuple2StringListU8TT, body []byte) {
	headers := hello_cosmo.WasiHttpHttpTypesNewFields(inHeaders)

	outgoingResponse := hello_cosmo.WasiHttpHttpTypesNewOutgoingResponse(statusCode, headers)
	if outgoingResponse.IsErr() {
		return
	}

	outgoingStream := hello_cosmo.WasiHttpHttpTypesOutgoingResponseWrite(outgoingResponse.Unwrap())
	if outgoingStream.IsErr() {
		return
	}

	w := hello_cosmo.WasiIoStreamsWrite(outgoingStream.Val, body)
	if w.IsErr() {
		return
	}

	hello_cosmo.WasiHttpHttpTypesFinishOutgoingStream(outgoingStream.Val)

	outparm := hello_cosmo.WasiHttpHttpTypesSetResponseOutparam(responseOutparam, outgoingResponse)
	if outparm.IsErr() {
		return
	}
}

func contentTypeJsonHeaders() []hello_cosmo.WasiHttpHttpTypesTuple2StringListU8TT {
	return []hello_cosmo.WasiHttpHttpTypesTuple2StringListU8TT{{F0: "Content-Type", F1: []byte("application/json")}}
}

func contentTypeMimeHeaders(mimeTime []byte) []hello_cosmo.WasiHttpHttpTypesTuple2StringListU8TT {
	return []hello_cosmo.WasiHttpHttpTypesTuple2StringListU8TT{{F0: "Content-Type", F1: mimeTime}}
}

func methodAndPath(request hello_cosmo.WasiHttpIncomingHandlerIncomingRequest) (hello_cosmo.WasiHttpHttpTypesMethod, hello_cosmo.Option[string]) {
	return hello_cosmo.WasiHttpHttpTypesIncomingRequestMethod(request), hello_cosmo.WasiHttpHttpTypesIncomingRequestPathWithQuery(request)
}
