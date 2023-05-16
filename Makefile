LD_SCRIPT_PATH := .cargo/linker-qemu.ld
KERNEL_ELF = target/aarch64-unknown-none-softfloat/release/test-os-on-orangepi-zero2
KERNEL_BIN = kernel.bin
QEMU_EXEC := qemu-system-aarch64 \
		-machine raspi3b \
		-kernel $(KERNEL_ELF) \
		-cpu cortex-a53 \
		-m 1G \
		-nographic \
		-smp 4
MKIMAGE := ~/Downloads/u-boot-orangepi/tools/mkimage
LOG := info
all: build

build:
	@LOG=$(LOG) cargo build --release
	rust-objcopy --strip-all -O binary $(KERNEL_ELF) $(KERNEL_BIN)

qemu: build
	@$(QEMU_EXEC)

debug: build
	@tmux new-session -d \
	"$(QEMU_EXEC) -s -S && echo '按任意键继续' && read -n 1" && \
	tmux split-window -h "aarch64-linux-gnu-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
	tmux -2 attach-session -d

gdb:
	aarch64-linux-gnu-gdb \
        -ex 'file $(KERNEL_ELF)' \
        -ex 'set arch riscv:rv64' \
        -ex 'target remote localhost:1234'

sdcard: build
	sudo mount /dev/mmcblk0p1 mount -o uid=1000,gid=1000
	$(MKIMAGE) -C none -A arm -T script -d boot/boot.cmd mount/boot.scr
	cp boot/orangepiEnv.txt mount
	cp kernel.bin mount
	sudo umount /dev/mmcblk0p1

flash: build
	sudo sunxi-fel spl u-boot-sunxi-with-spl.bin
	sudo sunxi-fel write 0x40080000 kernel.bin
	sudo sunxi-fel uboot u-boot-sunxi-with-spl.bin

miniterm:
	@sudo chmod 777 /dev/ttyUSB0
	python -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct /dev/ttyUSB0 115200