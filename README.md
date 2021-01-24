# Chapter 7 数据持久化（优先级1）
## 完成情况
* 实现了新的系统调用。可以创建或打开文件。
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
mod memory initialized
mod trap initialized
mod driver initialized
mod fs initialized
Hello World!

panic: 'app end'
```
### 主要动机
    实现数据持久化存储。
### 用户程序
    多种不同大小的文件读写。
### 内核应完成功能
    实现另一种在块设备上持久化存储的文件。文件系统不需要实现目录。
### 新增系统调用
* sys_open：创建或打开一个文件