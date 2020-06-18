## Setup all variables
RUSTUP = rustup
NIGHTLY_TOOLCHAIN = nightly
DOCKER = docker
CARGO = cargo

INSTALLED_TOOLCHAINS = $(shell $(RUSTUP) toolchain list)
INSTALLED_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(NIGHTLY_TOOLCHAIN))
INSTALLED_NIGHTLY_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(NIGHTLY_TOOLCHAIN))

## Dev environment
install_rust_nightly:
ifeq (,$(findstring $(NIGHTLY_TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(NIGHTLY_TOOLCHAIN)
endif

install_clippy: install_rust_nightly
ifeq (,$(findstring clippy,$(INSTALLED_COMPONENTS)))
	$(RUSTUP) component add clippy --toolchain $(NIGHTLY_TOOLCHAIN)
endif

install_rustfmt: install_rust_nightly
ifeq (,$(findstring rustfmt,$(INSTALLED_NIGHTLY_COMPONENTS)))
	$(RUSTUP) component add rustfmt --toolchain $(NIGHTLY_TOOLCHAIN)
endif

## Initial setup

setup: install_rust_nightly install_clippy install_rustfmt

## Development tasks

all: fmt build clippy test

build: install_rust_nightly
	$(CARGO) build --workspace --all-targets $(BUILD_ARGS)

clippy: install_clippy
	$(CARGO) clippy --all-targets -- -D warnings

test:
	$(DOCKER) build --tag rpll_backend_test_db ./Database/
	$(CARGO) test --workspace --all-targets --no-fail-fast

fmt: install_rustfmt
	$(CARGO_NIGHTLY) fmt -- --files-with-diff