PROFILE = debug
BUILD_FLAGS = 
BUILD_TARGET = thumbv7m-none-eabi
BINARY = stm32f10x

.PHONY = all
all: debug

.PHONY = debug
debug:
	$(MAKE) \
		PROFILE=debug \
		build

.PHONY = release
release:
	$(MAKE) \
		PROFILE=release \
		BUILD_FLAGS=--release \
		build

build: $(BINARY)
	arm-none-eabi-objcopy \
		-O binary \
		target/thumbv7m-none-eabi/$(PROFILE)/$(BINARY) \
		target/thumbv7m-none-eabi/$(PROFILE)/$(BINARY).bin

$(BINARY): Cargo.toml src/**/*.rs
	CARGO_BUILD_TARGET=$(BUILD_TARGET) \
		cargo build $(BUILD_FLAGS)
