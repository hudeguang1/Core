# Chapter1 裸机应用（优先级1）
## 完成情况
* 使用link.ld文件完成内核各个段进行布局。
* 移除标准库和运行时环境依赖，成功编译为裸机目标。
* 可以实现裸机运行两个要求的app。内核通过调用opensbi的接口实现串口输出。
* 两个系统调用暂时未实现。
```
OpenSBI v0.6
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 120 KB
Runtime SBI Version    : 0.2

MIDELEG : 0x0000000000000222
MEDELEG : 0x000000000000b109
PMP0    : 0x0000000080000000-0x000000008001ffff (A)
PMP1    : 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
text_start = 0x80200000, text_end= 0x80203000
rodata_start = 0x80203000, rodata_end= 0x80204000
data_start = 0x80204000, data_end= 0x80204000
bss_start = 0x80214000, bss_end= 0x80214000
Hello World!
sum = 15
```
## 主要动机
支持应用进行计算与结果输出。

在裸机上输出 Hello world，就像在其他 OS 上一样。

### app列表：
* hello_world：输出字符串

* count_sum：累加一维数组的和，并输出结果

**备注：不需要输入功能**

