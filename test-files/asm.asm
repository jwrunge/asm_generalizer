SECTION .data
msg     db      'Hello World!'

SECTION .text
global _start

_start:
	mov		edx, msg
	call	strlen

	; mov		edx, eax
	mov		ecx, msg
	mov		ebx, 1
	mov		eax, 4
	int		0x80

	mov		eax, 1
	mov		ebx, 0
	int		0x80

strlen:
	push   	rbx				; push ebx onto the stack
	mov		ebx, edx

nextchar:
	cmp		byte [edx], 0
	jz		finished
	inc		edx
	jmp		nextchar

finished:
	sub		edx, ebx
	pop		rbx				; pop value on the stack back into ebx
	ret						; return to the calling function
