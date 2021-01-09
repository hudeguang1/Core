
    .align 4
    .section .data
    .global _num_app
_num_app:
    .quad 5
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_4_end

    .global _app_names
_app_names:
    .string "00power_3\n"
    .string "01power_5\n"
    .string "02power_7\n"
    .string "03sleep\n"
    .string "04hello\n"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 12
app_0_start:
    .incbin "../user/target/riscv64imac-unknown-none-elf/release/00power_3"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 12
app_1_start:
    .incbin "../user/target/riscv64imac-unknown-none-elf/release/01power_5"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 12
app_2_start:
    .incbin "../user/target/riscv64imac-unknown-none-elf/release/02power_7"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 12
app_3_start:
    .incbin "../user/target/riscv64imac-unknown-none-elf/release/03sleep"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 12
app_4_start:
    .incbin "../user/target/riscv64imac-unknown-none-elf/release/04hello"
app_4_end:
