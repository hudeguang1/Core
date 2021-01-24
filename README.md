# Chapter 6 文件系统与进程间通信（优先级1）
## 完成情况
* 复制rCore Toturial中的文件系统和驱动代码。
* 完成rCore文件系统的加载和使用，可以打印指定目录下的文件名称。
* 完成新增系统调用，可以实现父进程写，子进程读的简单通信测试。
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
child
parent
hello_world
panic: 'app end'
```
## 问题
* 由于内存模块中将设备段的映射删除，所以在exec加载文件时，出现StorePageFault错误。已解决
* close函数有一些问题，父进程读端口关闭时，子进程也会关闭。
### 主要动机
    进程之间需要进行一些协作。本章主要是通过管道进行通信。同时，需要引入文件系统，并通过文件描述符来访问对应类型的 Unix 资源。
### 用户程序
    简单的通过 fork 和子进程共享管道的测试；
### 内核应完成功能
    实现管道。将字符设备（标准输入/输出）和管道封装为通过文件描述符访问的文件。
### 新增系统调用
* sys_pipe：目前对于管道的 read/write 只需实现轮询版本。
* sys_close：作用是关闭管道