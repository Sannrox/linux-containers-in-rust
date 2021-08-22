.PHONY: build-docker start-docker run


build-docker:
	./scripts/docker/build-container

start-docker:
	./scripts/docker/run-container

run:
	cargo run
