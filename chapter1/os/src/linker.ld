/*
一般来说，一个程序按照功能不同会分为下面这些段：
(低地址)

.text 段：代码段，存放汇编代码
.rodata 段：只读数据段，顾名思义里面存放只读数据，通常是程序中的常量
.data 段：存放被初始化的可读写数据，通常保存程序中的全局变量
.bss 段：存放被初始化为 0 的可读写数据，与 .data 段的不同之处在于
	我们知道它要被初始化为 0，因此在可执行文件中只需记录这个段的大小以及所在位置即可，
	而不用记录里面的数据，也不会实际占用二进制文件的空间
Heap：堆，用来支持程序运行过程中内存的动态分配，比如说你要读进来一个字符串，
	在你写程序的时候你也不知道它的长度究竟为多少，于是你只能在运行过程中，
	知道了字符串的长度之后，再在堆中给这个字符串分配内存
Stack：栈，用来存储程序运行过程中的局部变量，以及负责函数调用时的各种机制。
	它从高地址向低地址增长

（高地址）
*/



/*目标架构*/
OUTPUT_ARCH(riscv)

/*执行入口*/
ENTRY(_start)

/*数据存放的起始地址*/
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }
    . = ALIGN(4k);
	text_end = .;
    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }
    . = ALIGN(4k);
	rodata_end = .;

    data_start = .;
    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }
    . = ALIGN(4k);
	data_end = .;

    
    /* .bss 字段 */
    .bss : {
        *(.bss.stack)
        /* 要链接的文件的 .bss 字段集中放在这里 */
        bss_start = .;
        *(.bss .bss.*)
        
    }
    bss_end = .;
    . = ALIGN(4k);
	
    /* 结束地址 */
    kernel_end = .;
}