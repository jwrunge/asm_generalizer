SECTION .data
msg     db      'Hello World!', 0Ah

SECTION .text
global _start

_start:

	mov	eax, 4
	mov	ebx, 1
	mov	ecx, msg
	mov	edx, 13
	int	0x80
