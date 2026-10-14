RC := cargo
RFLAGS := --verbose
RELEASE := --release
VARS := RUST_BACKTRACE=full

.PHONY: run
run:
	@echo "Running project >>>"
	@$(VARS) $(RC) $(RFLAGS) run

.PHONY: test
test:
	@echo "Running tests >>>"
	@$(VARS) $(RC) $(RFLAGS) test

.PHONY: build
build:
	@echo "Building project >>>"
	@$(VARS) $(RC) $(RFLAGS) build

.PHONY: build-release
build-release:
	@echo "Building project in release mode >>>"
	@$(VARS) $(RC) $(RFLAGS) build $(RELEASE)
