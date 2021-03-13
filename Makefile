arch ?= x86_64
target ?= $(arch)-yaos
rust_os := target/$(target)/debug/libyaos.a
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/kernel/arch/$(arch)/linker.ld
grub_cfg := src/kernel/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/kernel/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/kernel/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel check

all: $(kernel)

clean:
	@rm -rf build
	@cargo clean

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -serial stdio

#-d int -no-reboot -no-shutdown
debug: $(iso)
	@/bin/bash -c '/usr/bin/killall -q qemu-system-x86_64; exit 0'
	@qemu-system-x86_64 -cdrom $(iso) -s -S -serial stdio&
	@sleep 1
	@rust-gdb $(kernel)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel:
	@cargo build

check:
	@cargo check

format:
	@./scripts/format_code.sh

# compile assembly files
build/arch/$(arch)/%.o: src/kernel/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
