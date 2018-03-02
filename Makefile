ARMGNU ?= arm-none-eabi

CC = $(ARMGNU)-gcc
AS = $(ARMGNU)-as
LD = $(ARMGNU)-ld
OD = $(ARMGNU)-objdump
OC = $(ARMGNU)-objcopy

all : kernel.img


boot.o: boot.S
	$(AS) boot.S -o boot.o

target/arm-none-eabihf/debug/libpizos.a: src/lib.rs src/hal/mod.rs arm-none-eabihf.json
	xargo build --verbose --target arm-none-eabihf

clean:
	rm -vf target/arm-none-eabihf/debug/libpizos.a boot.o kernel.elf kernel.img kernel.bin kernel.hex kernel.list

#kernel.img : loader boot.o target/arm-none-eabihf/debug/libpizos.rlib target/arm-none-eabihf/debug/deps/libcompiler_builtins-26fdd3bba9d26080.rlib /home/ubuntu/.xargo/lib/rustlib/arm-none-eabihf/lib/libcore-a07bb5458544daef.rlib
#	$(LD) --gc-sections boot.o target/arm-none-eabihf/debug/libpizos.rlib target/arm-none-eabihf/debug/deps/libcompiler_builtins-26fdd3bba9d26080.rlib /home/ubuntu/.xargo/lib/rustlib/arm-none-eabihf/lib/libcore-a07bb5458544daef.rlib -T loader -o kernel.elf
kernel.img : loader boot.o target/arm-none-eabihf/debug/libpizos.a
	$(LD) --gc-sections boot.o target/arm-none-eabihf/debug/libpizos.a -T loader -o kernel.elf
	$(OD) -D kernel.elf > kernel.list
	$(OC) kernel.elf -O ihex kernel.hex
	$(OC) kernel.elf -O binary kernel.bin
	$(OC) kernel.elf -O binary kernel.img
