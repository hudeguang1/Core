# Chapter 5 进程及重要系统调用（优先级1）
## 完成情况
* 实现新增的系统调用。
* 通过了简单的测试用例，吴一凡同学的一些测试无法通过。
* 问题：对于fork和exec功能起初没有太明确，通过询问和查阅资料了解。fork复制过程缓慢，未进行优化。没有实现公用的内核栈。
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
Children
Parent
son process all end
pid:1
hello world
power_3 [10000/30000]
power_3 [20000/30000]
power_3 [30000/30000]
3^30000 = 813516892(mod 998244353)
Test power_3 OK!
panic: 'app end'
```
### 主要动机
      应用以进程的方式进行运行，简化了应用开发的负担，OS也更好管理。引入重要的进程概念，整合Chapt1~4的内容抽象出进程，实现一系列相关机制及 syscall
### 用户程序
      用户终端 user_shell 以及一些相应的测试
### 内核应完成功能
      实现完整的子进程机制，初始化第一个用户进程 initproc。
      新增系统调用
      sys_fork
      sys_wait(轮询版)
      sys_exec
      sys_getpid
      sys_yield更新
      sys_exit 更新
      sys_read：终端需要从串口读取命令