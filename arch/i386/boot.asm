global start
global stack_top
global stack_bottom

section .text
bits 32
start:
	mov esp, stack_top
	extern kmain
	call kmain

section .bss
stack_bottom:
	resb 0x10000
stack_top:
