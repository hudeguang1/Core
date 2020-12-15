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
	text_end = .;

    rodata_start = .;
    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }
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
	
    /* 结束地址 */
    kernel_end = .;
}