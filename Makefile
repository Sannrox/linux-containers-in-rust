.PHONY: build-docker start-docker run shellcheck help


build-docker: ## build the main container
	./scripts/docker/build-container

shellcheck: ## start the shellcheck
	docker buildx bake shellcheck

start-docker: ## start the docker
	./scripts/docker/run-container

run: warning-outside ## run
	cargo run

lint: warning-outside ## lint
	cargo clippy

fmt: warning-outside ## format
	cargo fmt

warning-outside:


help: ## print this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)


