# Cosmonic WasmCon Workshop

Is it possible to sketch an idea for an application on a napkin and, in a few clicks, have it running live in multi-cloud, multi-edge, far-edge environments? And is it possible to run them at near native speeds? In this workshop, we’ll show all this is possible – and more! Developers will get up close and personal with the Cosmonic PaaS: the fast, secure-by-default, distributed application development platform that eliminates entire classes of development challenges and dissolves management costs. These hands-on demonstrations will bring WebAssembly to life and demonstrate the speed and ease of going from simple sketch to rapid scale with WebAssembly and Cosmonic.

## What to Expect

- Go from beginner to expert on Cosmonic.
- Demo: a deep dive under the hood of the Cosmonic PaaS.
- Cosmonic Platform-as-a-Service and Q&A.
- Meet some of the core contributors to wasmCloud and other core Wasm projects: Taylor Thomas and Bailey Hayes.
- Hands-on with Cosmonic and wasmCloud.
- Build several applications.
- Deploy your applications to Cosmonic and your own infrastructure simultaneously.

## How to use this Repository

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=685628224&machine=standardLinux32gb&location=EastUs)

Working in this repository is best done using the packaged [devcontainer](./.devcontainer) which includes all of the necessary toolchains and binaries to follow along. You can either use the button above to open this automatically in GitHub codespaces (free) or you can work on this repository using local tools with the below **prerequisites**:

1. [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
1. [Docker](https://docs.docker.com/engine/install/)
1. [VSCode](https://code.visualstudio.com/download) and the [devcontainer extension]([https://code.visualstudio.com/docs/devcontainers/devcontainer-cli#_installation](https://code.visualstudio.com/docs/devcontainers/containers#_installation))

To work locally, after installing the prerequisites, clone this repository and open the devcontainer at the root.

```bash
git clone https://github.com/cosmonic/wasmcon-workshop.git
cd wasmcon-workshop
devcontainer open
```

2. cosmo login
3. just build && just push
4. edit wadm.yaml
5. cosmo app put wadm.yaml
6. cosmo app deploy hello-cosmo
