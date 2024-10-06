.PHONY: all build-linux build-windows copy-linux copy-windows package-linux package-windows clean
# vim: set list expandtab ts=4 sw=4:

BIN_NAME = dorf-name
DATA_FILES = data
LINUX_TARGET = x86_64-unknown-linux-gnu
WINDOWS_TARGET = x86_64-pc-windows-gnu
DIST_DIR = dist
GIT_TAG = $(shell git describe --tags --always)

# Default target that builds and packages everything
all: build-linux build-windows package-linux package-windows

# Build the project for Linux
build-linux:
	cargo build --release --target $(LINUX_TARGET)

# Build the project for Windows
build-windows:
	cargo build --release --target $(WINDOWS_TARGET)

# Copy files and create the Linux distribution directory
copy-linux:
	mkdir -p $(DIST_DIR)/linux
	cp target/$(LINUX_TARGET)/release/$(BIN_NAME) $(DIST_DIR)/linux/
	cp -r $(DATA_FILES) $(DIST_DIR)/linux/

# Copy files and create the Windows distribution directory
copy-windows:
	mkdir -p $(DIST_DIR)/windows
	cp target/$(WINDOWS_TARGET)/release/$(BIN_NAME).exe $(DIST_DIR)/windows/
	cp -r $(DATA_FILES) $(DIST_DIR)/windows/

# Package the Linux distribution into a tarball with Git tag in the name
package-linux: copy-linux
	tar -czvf $(DIST_DIR)/$(BIN_NAME)-linux-$(GIT_TAG).tar.gz -C $(DIST_DIR)/linux .

# Package the Windows distribution into a zip file with Git tag in the name
package-windows: copy-windows
	zip -r $(DIST_DIR)/$(BIN_NAME)-windows-$(GIT_TAG).zip $(DIST_DIR)/windows/*

# Clean up the dist directory
clean:
	rm -rf $(DIST_DIR)
