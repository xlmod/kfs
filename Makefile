
# INFO
VERSION=0.1.0
KERNEL=kfs-${VERSION}
ARCH=i386

# RUN VAR
QEMU=qemu-system-i386
QEMUFLAGS=

# DIRECTORY SOURCE
DIR_SRC=src
DIR_ARCH=arch

# DIRECTORY BUILD
DIR_BUILD=build
DIR_ISO=iso
DIR_ISO_BOOT=${DIR_ISO}/boot
DIR_ISO_GRUBCONF=${DIR_ISO_BOOT}/grub

# BUILD VAR
AS=nasm
ASFLAGS=-f elf32
LD=ld
LDFLAGS=-m elf_i386 -n -T ${DIR_ARCH}/${ARCH}/linker.ld


all: bootable

kernel: ${DIR_ISO_BOOT}/${KERNEL}

grubconf:
	@mkdir -p ${DIR_ISO_GRUBCONF}
	sed 's/__kfs__/${KERNEL}/' ${DIR_ARCH}/${ARCH}/grub.cfg > ${DIR_ISO_GRUBCONF}/grub.cfg

${DIR_BUILD}/multiboot_header.o: ${DIR_ARCH}/${ARCH}/multiboot_header.asm
	@mkdir -p ${DIR_BUILD}
	${AS} ${ASFLAGS} -o $@ $^

${DIR_BUILD}/boot.o: ${DIR_ARCH}/${ARCH}/boot.asm
	@mkdir -p ${DIR_BUILD}
	${AS} ${ASFLAGS} -o $@ $^

${DIR_ISO_BOOT}/${KERNEL}: ${DIR_BUILD}/boot.o ${DIR_BUILD}/multiboot_header.o
	@mkdir -p ${DIR_ISO_BOOT}
	${LD} ${LDFLAGS} -o $@ $^

bootable: grubconf kernel
	grub2-mkrescue --compress=xz -o ${KERNEL} ${DIR_ISO}

run: bootable
	${QEMU} ${QEMUFLAGS} -drive format=raw,file=${KERNEL}


.PHONY: all kernel grubconf bootable run
