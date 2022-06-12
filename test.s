# made using `clang -S -mllvm --x86-asm-syntax=intel test.c`
	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0
	.intel_syntax noprefix
	.globl	_main                           ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset rbp, -16
	mov	rbp, rsp
	.cfi_def_cfa_register rbp
	mov	dword ptr [rbp - 4], 0
	mov	eax, 1
	pop	rbp
	ret
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
