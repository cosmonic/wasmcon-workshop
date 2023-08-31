Build  && Run steps

Requirements:
- [tinygo](https://github.com/tinygo-org/tinygo)
- [wit-deps](https://github.com/bytecodealliance/wit-deps)
- [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
- [just](https://github.com/casey/just) 
- [wasmcloud](https://github.com/wasmcloud/wasmcloud)
- [wash](https://github.com/wasmcloud/wash)

```
wit-deps
just build

export ACTOR_ID=<actorid>
export HOST_ID=<hostid> # of Rust host 

just start_actor
```

Versions used for this demo

```
└─❯ just versions
tinygo version 0.28.1 darwin/arm64 (using go version go1.20.7 and LLVM version 15.0.7)
wash 0.19.1
wasm-tools 1.0.38 (9fb2019dc 2023-08-18)
wit-bindgen-cli 0.9.0 (8b3e23e8f 2023-08-18)
```
