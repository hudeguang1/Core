# 操作系统启动时所需的指令以及字段
# 
# 我们在linker.ld中将程序入口设置为_start，在这里将填充这个标签
# 执行一些必要操作，然后跳转到用rust写的入口函数

    .section .text.entry
    .global _start
# _start的功能：将预留的栈空间写入$sp，然后跳转至rust_main
# 1、修改栈指针寄存器 sp 为 .bss.stack 段的结束地址，
#    由于栈是从高地址往低地址增长，所以高地址是初始的栈顶；
# 2、使用 call 指令跳转到 rust_main 。这意味着我们的内核运行环境设置完成了，
#    正式进入内核。
_start:
    la sp, boot_stack_top
    call rust_main

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 启动栈大小 64
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    #栈结尾