
# INFO
version:=0.1.0
arch:=i386


# RUN VAR
qemu:=qemu-system-i386

# DIRECTORY SOURCE
dir_src:=src
dir_arch:=arch

# DIRECTORY BUILD
dir_build:=build
dir_iso:=${dir_build}/iso
dir_iso_boot:=${dir_iso}/boot
dir_iso_grub:=${dir_iso_boot}/grub

# FILES
linker_script:=${dir_arch}/${arch}/linker.ld
grub_cfg:=${dir_arch}/${arch}/grub.cfg
assembly_source_files:=$(wildcard ${dir_arch}/${arch}/*.asm)
assembly_object_files:=$(patsubst ${dir_arch}/${arch}/%.asm, \
					   ${dir_build}/${dir_arch}/${arch}/%.o, \
					   ${assembly_source_files})

kernelname:=kfs-${version}
kernel:=${dir_build}/${kernelname}.bin
iso:=${dir_build}/${kernelname}



# BUILD VAR
AS:=nasm
ASFLAGS:=-f elf32
LD:=ld
LDFLAGS:=-m elf_i386 -n -T ${linker_script}
GRUBMK:=grub2-mkrescue
GRUBMKFLAGS:=--compress=xz


.PHONY: all clean re run iso

all: ${kernel}

clean:
	${RM} -r ${dir_build}

re: clean all

run: ${iso}
	${qemu} -drive format=raw,file=${iso}

${iso}: ${kernel} ${grub_cfg}
	mkdir -p ${dir_iso_grub}
	cp ${kernel} ${dir_iso_boot}/${kernelname}
	sed 's/__kfs__/${kernelname}/' ${grub_cfg} > ${dir_iso_grub}/grub.cfg
	${GRUBMK} ${GRUBMKFLAGS} -o ${iso} ${dir_iso}

${kernel}: ${assembly_object_files}
	${LD} ${LDFLAGS} -o $@ ${assembly_object_files}

${dir_build}/${dir_arch}/${arch}/%.o: ${dir_arch}/${arch}/%.asm
	@mkdir -p $(shell dirname $@)
	${AS} ${ASFLAGS} $< -o $@
