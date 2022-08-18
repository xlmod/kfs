global start

section .text
bits 32
start:
	mov esp, stack_top
	extern kmain
	call kmain
	hlt

section .bss
stack_bottom:
	resb 128
stack_top:
