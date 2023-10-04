IMAGE=noggin

.PHONY: build-image
build-image:
	@docker build -t "$(IMAGE)" -f ./dev.docker .

.PHONY: build-dev
build-dev: build-image
	$(MAKE) build-pre-commit
	cp ./docker/pre-commit-hook.sh .git/hooks/pre-commit

.PHONY: build-pre-commit
build-pre-commit:
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit install-hooks

.PHONY: test
test: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh cargo test

.PHONY: pre-commit
pre-commit: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit run --color=always

.PHONY: pre-commit-all
pre-commit-all: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit run --all-files --color=always

.PHONY: shell
shell: build-dev
	@IMAGE=$(IMAGE) ./docker/run.sh bash

.PHONY: build-image-ci
build-image-ci: build-image
	docker save "$(IMAGE)" | gzip > ./build-cache/docker-image.tar.gz

.PHONY: build-tests
build-tests:
	IMAGE=$(IMAGE) ./docker/run.sh cargo build --tests

.PHONY: load-image-ci
load-image-ci:
	@gunzip -c ./build-cache/docker-image.tar.gz | docker load

.PHONY: test-ci
test-ci: load-image-ci
	@IMAGE=$(IMAGE) ./docker/run.sh cargo test

.PHONY: pre-commit-ci
pre-commit-ci: load-image-ci
	@IMAGE=$(IMAGE) ./docker/run.sh pre-commit run --all-files --color=always
