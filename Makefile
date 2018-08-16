arch ?= x86_64
bin := build/hello-$(arch).bin
target ?= $(arch)-hello
hello := target/$(target)/debug/libhello.a

linker_script := src/arch/$(arch)/linker.ld
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean

all: $(bin)

clean:
	@rm -r build

$(bin): hello $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(bin) $(assembly_object_files) $(hello)

hello:
	@xargo build --target $(target)

# Compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@