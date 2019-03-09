EXAMPLE := blinky

ARCH := thumbv7em-none-eabihf

BUILD := debug
VALID_BUILDS := debug release
ifeq ($(filter $(BUILD),$(VALID_BUILDS)),)
$(error BUILD must be either debug or release)
endif

BOARD := solo
VALID_BOARDS := solo nucleo
ifeq ($(filter $(BUILD),$(VALID_BUILDS)),)
$(error BOARD must be either solo or nucleo)
else
$(shell ln -sf memory-$(BOARD).x memory.x)
endif

ifeq ($(BUILD),release)
RELEASE_FLAG := --release
endif

ELF := target/$(ARCH)/$(BUILD)/examples/$(EXAMPLE)
HEX := $(EXAMPLE)-$(BUILD).hex

example:
	BOARD=$(BOARD) cargo build --example $(EXAMPLE) $(RELEASE_FLAG)
	cargo size --example $(EXAMPLE) $(RELEASE_FLAG) -- -A|grep -v .debug|grep -v Total
	arm-none-eabi-objcopy -O ihex $(ELF) $(HEX)

# For Solo
flash-solo:
	solo program aux enter-bootloader
	solo program bootloader $(HEX)

# For Nucleo
openocd:
	openocd

gdb:
	gdb-multiarch -q -x openocd.gdb $(ELF)
