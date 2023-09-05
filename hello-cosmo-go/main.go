package main

import (
	hello_cosmo "hello_cosmo/gen"
)

type MyHelloCosmo struct{}

func (kv *MyHelloCosmo) Handle(request hello_cosmo.WasiHttpIncomingHandlerIncomingRequest, response hello_cosmo.WasiHttpHttpTypesResponseOutparam) {

}

func init() {
	myhc := new(MyHelloCosmo)
	hello_cosmo.SetExportsWasiHttpIncomingHandler(myhc)
}

//go:generate wit-bindgen tiny-go ./wit -w hello-cosmo --out-dir=gen
func main() {}
