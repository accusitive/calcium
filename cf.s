	.text
	.file	"module"
	.globl	std__newstring                  # -- Begin function std__newstring
	.p2align	4, 0x90
	.type	std__newstring,@function
std__newstring:                         # @std__newstring
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %r14
	callq	strlen@PLT
	movq	%rax, %rbx
	movl	$.Lglobal_string, %edi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$16, %edi
	callq	malloc@PLT
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	movq	%rbx, %rax
	movq	%r14, %rdx
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	std__newstring, .Lfunc_end0-std__newstring
	.cfi_endproc
                                        # -- End function
	.globl	std__booltostr                  # -- Begin function std__booltostr
	.p2align	4, 0x90
	.type	std__booltostr,@function
std__booltostr:                         # @std__booltostr
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	xorl	%eax, %eax
	movl	$16, %edi
	testb	%al, %al
	je	.LBB1_1
# %bb.2:                                # %then
	callq	malloc@PLT
	movq	$4, (%rax)
	movq	$.Lglobal_string.1, 8(%rax)
	movl	$4, %eax
	movl	$.Lglobal_string.1, %edx
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB1_1:                                # %else
	.cfi_def_cfa_offset 16
	callq	malloc@PLT
	movq	$5, (%rax)
	movq	$.Lglobal_string.2, 8(%rax)
	movl	$5, %eax
	movl	$.Lglobal_string.2, %edx
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	std__booltostr, .Lfunc_end1-std__booltostr
	.cfi_endproc
                                        # -- End function
	.globl	std__assert                     # -- Begin function std__assert
	.p2align	4, 0x90
	.type	std__assert,@function
std__assert:                            # @std__assert
	.cfi_startproc
# %bb.0:                                # %entry
	testb	$1, %dil
	jne	.LBB2_2
# %bb.1:                                # %else
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$1, %edi
	callq	std__booltostr@PLT
	movl	$1, %edi
	callq	exit@PLT
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
.LBB2_2:                                # %cont
	xorl	%eax, %eax
	retq
.Lfunc_end2:
	.size	std__assert, .Lfunc_end2-std__assert
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	cmpl	$1, %edi
	jg	.LBB3_1
# %bb.4:                                # %cont
	xorl	%eax, %eax
	retq
.LBB3_1:                                # %then
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movl	%edi, %ebx
	movl	$.Lglobal_string.3, %edi
	xorl	%eax, %eax
	callq	printf@PLT
	cmpl	$5, %ebx
	jle	.LBB3_2
# %bb.3:                                # %then2
	movl	$.Lglobal_string.4, %edi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB3_2:                                # %cont4
.Lfunc_end3:
	.size	main, .Lfunc_end3-main
	.cfi_endproc
                                        # -- End function
	.globl	print                           # -- Begin function print
	.p2align	4, 0x90
	.type	print,@function
print:                                  # @print
	.cfi_startproc
# %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	subq	$224, %rsp
	testb	%al, %al
	je	.LBB4_4
# %bb.3:
	movaps	%xmm0, -176(%rbp)
	movaps	%xmm1, -160(%rbp)
	movaps	%xmm2, -144(%rbp)
	movaps	%xmm3, -128(%rbp)
	movaps	%xmm4, -112(%rbp)
	movaps	%xmm5, -96(%rbp)
	movaps	%xmm6, -80(%rbp)
	movaps	%xmm7, -64(%rbp)
.LBB4_4:
	movq	%r9, -184(%rbp)
	movq	%r8, -192(%rbp)
	movq	%rcx, -200(%rbp)
	movq	%rdx, -208(%rbp)
	movq	%rsi, -216(%rbp)
	movq	%fs:40, %rax
	movq	%rax, -8(%rbp)
	movq	%rdi, -40(%rbp)
	movl	$.L.str, %edi
	xorl	%eax, %eax
	callq	printf@PLT
	leaq	-224(%rbp), %rax
	movq	%rax, -16(%rbp)
	leaq	16(%rbp), %rax
	movq	%rax, -24(%rbp)
	movl	$48, -28(%rbp)
	movl	$8, -32(%rbp)
	movq	-40(%rbp), %rdi
	leaq	-32(%rbp), %rsi
	callq	vprintf@PLT
	movq	%fs:40, %rax
	movq	-8(%rbp), %rcx
	cmpq	%rcx, %rax
	jne	.LBB4_2
# %bb.1:                                # %SP_return
	movl	$5, %eax
	addq	$224, %rsp
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB4_2:                                # %CallStackCheckFailBlk
	.cfi_def_cfa %rbp, 16
	callq	__stack_chk_fail@PLT
.Lfunc_end4:
	.size	print, .Lfunc_end4-print
	.cfi_endproc
                                        # -- End function
	.type	.Lglobal_string,@object         # @global_string
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lglobal_string:
	.asciz	"String len is %i\n"
	.size	.Lglobal_string, 18

	.type	.Lglobal_string.1,@object       # @global_string.1
.Lglobal_string.1:
	.asciz	"true"
	.size	.Lglobal_string.1, 5

	.type	.Lglobal_string.2,@object       # @global_string.2
.Lglobal_string.2:
	.asciz	"false"
	.size	.Lglobal_string.2, 6

	.type	.Lglobal_string.3,@object       # @global_string.3
.Lglobal_string.3:
	.asciz	"> 1\n"
	.size	.Lglobal_string.3, 5

	.type	.Lglobal_string.4,@object       # @global_string.4
.Lglobal_string.4:
	.asciz	"MORE THAN 5 !"
	.size	.Lglobal_string.4, 14

	.type	.L.str,@object                  # @.str
.L.str:
	.asciz	"PRINT\n"
	.size	.L.str, 7

	.ident	"clang version 12.0.1"
	.section	".note.GNU-stack","",@progbits
