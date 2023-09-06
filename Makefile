DEFAULT_GOAL := help
IMAGE_NAME ?= ghcr.io/cosmonic/wasmcon-workshop
PLATFORM ?= linux/amd64,linux/arm64
TAG ?= latest

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

docker: ## Build and push the workshop image for multiple architectures using buildx
	-@docker buildx create --driver-opt=network=host --name multiarchbuilder --use
	@docker buildx inspect multiarchbuilder --bootstrap
	@docker buildx build --platform $(PLATFORM) -t $(IMAGE_NAME):$(TAG) . --push

run-docker: ## Run the workshop image in a regular container, copying in all files
	@docker run -it --rm -v $(PWD):/wasmcon-workshop --workdir /wasmcon-workshop $(IMAGE_NAME):$(TAG)

install-npm: ## Install NodeJS 18.x with NPM to modify the UI in the workshop
	@mkdir -p /etc/apt/keyrings && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
	@echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_18.x nodistro main" | sudo tee /etc/apt/sources.list.d/nodesource.list
	@sudo apt-get update && sudo apt-get install nodejs -y

.PHONY: docker
