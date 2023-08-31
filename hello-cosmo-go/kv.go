package main

import (
	"strconv"

	hello_cosmo "hello_cosmo/gen"
)

func IncrementCounter(bucket uint32, key string, amount int32) uint32 {
	var currentValue uint32
	currentValueGet := hello_cosmo.WasiKeyvalueReadwriteGet(bucket, key)
	if currentValueGet.IsErr() {
		currentValue = 0
	} else {
		b := hello_cosmo.WasiKeyvalueTypesIncomingValueConsumeSync(currentValueGet.Unwrap())
		if b.IsErr() {
			// TODO: We should have actual errors here
			return 0
		}
		bNum, err := strconv.Atoi(string(b.Unwrap()))
		if err != nil {
			return 0
		}
		currentValue = uint32(bNum)
	}

	newValue := currentValue + uint32(amount)
	outgoingValue := hello_cosmo.WasiKeyvalueTypesNewOutgoingValue()
	stream := hello_cosmo.WasiKeyvalueTypesOutgoingValueWriteBody(outgoingValue)
	if stream.IsErr() {
		return 0
	}

	hello_cosmo.WasiIoStreamsWrite(stream.Unwrap(), []byte(strconv.Itoa(int(newValue))))

	_ = hello_cosmo.WasiKeyvalueReadwriteSet(bucket, key, outgoingValue)
	// TODO: this is throwing an error even though it isn't erroring
	// if res.IsErr() {
	// 	return 103
	// }

	stat := hello_cosmo.WasiKeyvalueReadwriteGet(bucket, key)
	if stat.IsErr() {
		return 0
	}

	return newValue
}
