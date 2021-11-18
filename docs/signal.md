# Linux Signal 列表

### Linux 信号表 

Linux 支持 POSIX 标准信号和实时信号。下面给出 Linux Signal 的简表，详细细节可以查看 man 7 signal。

LINUX 信号 总共 64 (个) 使用 `kill -l` 查看支持信号列表
```shell
[root@localhost ~]# kill -l
 1) SIGHUP       2) SIGINT       3) SIGQUIT      4) SIGILL       5) SIGTRAP
 6) SIGABRT      7) SIGBUS       8) SIGFPE       9) SIGKILL     10) SIGUSR1
11) SIGSEGV     12) SIGUSR2     13) SIGPIPE     14) SIGALRM     15) SIGTERM
16) SIGSTKFLT   17) SIGCHLD     18) SIGCONT     19) SIGSTOP     20) SIGTSTP
21) SIGTTIN     22) SIGTTOU     23) SIGURG      24) SIGXCPU     25) SIGXFSZ
26) SIGVTALRM   27) SIGPROF     28) SIGWINCH    29) SIGIO       30) SIGPWR
31) SIGSYS      34) SIGRTMIN    35) SIGRTMIN+1  36) SIGRTMIN+2  37) SIGRTMIN+3
38) SIGRTMIN+4  39) SIGRTMIN+5  40) SIGRTMIN+6  41) SIGRTMIN+7  42) SIGRTMIN+8
43) SIGRTMIN+9  44) SIGRTMIN+10 45) SIGRTMIN+11 46) SIGRTMIN+12 47) SIGRTMIN+13
48) SIGRTMIN+14 49) SIGRTMIN+15 50) SIGRTMAX-14 51) SIGRTMAX-13 52) SIGRTMAX-12
53) SIGRTMAX-11 54) SIGRTMAX-10 55) SIGRTMAX-9  56) SIGRTMAX-8  57) SIGRTMAX-7
58) SIGRTMAX-6  59) SIGRTMAX-5  60) SIGRTMAX-4  61) SIGRTMAX-3  62) SIGRTMAX-2
63) SIGRTMAX-1  64) SIGRTMAX 
```

#### 默认动作的含义如下：

- Term 终止进程

| 信号    | 取值 | 默认动作 | 含义（发出信号的原因）|
| ----   | -----:|:------- | --------------------|
| 第一部分   |      |         |                     |
| SIGHUP    |1     | Term |  终端的挂断或进程死亡 |
| SIGINT    |2     | Term | 来自键盘的中断信号 |
| SIGQUIT    |3     | Core | 来自键盘的离开信号 |
| SIGILL    |4     | Core | 非法指令 |
| SIGABRT    |6     | Core  | 来自 abort 的异常信号 |
| SIGFPE    |8     | Core  | 浮点例外 |
| SIGKILL    |9     | Term  | 杀死 |
| SIGSEGV    |11     | Core | 段非法错误(内存引用无效) |
| SIGPIPE    |13     | Term | 管道损坏：向一个没有读进程的管道写数据 |
| SIGALRM    |14     | Term | 来自 alarm 的计时器到时信号 |
| SIGTERM    |15     | Term  | 终止 |
| SIGUSR1    |30, 10, 16 | Term|    用户自定义信号 1 |
| SIGUSR2    |31, 12, 17 | Term|    用户自定义信号 2 |
| SIGCHLD    |20, 17, 18 | Ign | 子进程停止或终止 |
| SIGCONT    |19, 18, 25 | Cont|    如果停止，继续执行 |
| SIGSTOP    |17, 19, 23 | Stop|    非来自终端的停止信号 |
| SIGTSTP    |18, 20, 24 | Stop|    来自终端的停止信号 |
| SIGTTIN    |21, 21, 26 | Stop|    后台进程读终端 |
| SIGTTOU    |22, 22, 27 | Stop|    后台进程写终端 |
| 第二部分    | | | |
| SIGBUS    |10, 7, 10    |Core |总线错误（内存访问错误）|
| SIGPOLL    |    |Term    |Pollable 事件发生(Sys V)，与 SIGIO 同义|
| SIGPROF    |27, 27, 29|Term|布图用计时器到时
| SIGSYS    |12, -, 12 |Core|非法系统调用(SVr4)|
| SIGTRAP    |5     |Core |跟踪/断点自陷|
| SIGURG    |16, 23, 21 |Ign    |socket 紧急信号(4.2BSD)|
| SIGVTALRM |26, 26, 28 |Term |虚拟计时器到时(4.2BSD)|
| SIGXCPU    |24, 24, 30|Core |超过 CPU 时限(4.2BSD)|
| SIGXFSZ    |25, 25, 31|Core |超过文件长度限制(4.2BSD)|****
| 第三部分    | | | |
| SIGIOT    |6    |Core    |IOT 自陷，与 SIGABRT 同义|
| SIGEMT    |7, -, 7|Term|  |
| SIGSTKFLT |-, 16, -| Term |协处理器堆栈错误(不使用)|
| SIGIO    |23, 29, 22| Term |描述符上可以进行 I/O 操作|
| SIGCLD    |-, -, 18| Ign |与 SIGCHLD 同义|
| SIGPWR    |29, 30, 19|Term|电力故障(System V)|
| SIGINFO    |29, -, -| |与 SIGPWR 同义|
| SIGLOST    |-, -, -| Term |文件锁丢失|
| SIGWINCH    |28, 28, 20| Ign |窗口大小改变(4.3BSD, Sun)|
| SIGUNUSED |-, 31, - |Term |未使用信号(will be SIGSYS)|

一些信号的取值是硬件结构相关的（一般 alpha 和 sparc 架构用第一个值，i386、ppc 和 sh 架构用中间值，mips 架构用第三个值，*-* 表示相应架构的取值未知）。

### 第一部分的是 POSIX.1-1990 标准信号。

SIGKILL 和 SIGSTOP 信号不能被挂钩、阻塞或忽略。

### 第二部分的是 SUSv2 和 POSIX.1-2001 定义的信号。

在 Linux 2.2（包括）内核之前，SIGSYS、SIGXCPU、SIGXFSZ 和 SIGBUS（SPARC 和 MIPS 架构除外）的默认动作是终止进程，但没有 core dump。Linux 2.4 遵循 POSIX.1-2001
要求，这些信号的默认动作改为：终止进程同时做 core dump。

### 第三部分的是其他常见的信号。

信号 29 在 Alpha 上为 SIGINFO / SIGPWR，在 Sparc 上为 SIGLOST。

SIGEMT 没有在 POSIX.1-2001 中说明，但是在大多数的 Unices 中仍然能见到，典型的默认动作是终止进程并做 core dump。

SIGPWR 没有在 POSIX.1-2001 中说明，在使用它的一些 Unices 中典型的默认动作是忽略。

SIGIO 没有在 POSIX.1-2001 中说明，在使用它的一些 Unices 中典型的默认动作是忽略。

进程可以通过使用 sigaction 和 signal 系统调用来改变信号的默认处理方式（使用 signal 的可移植性差）。进程可以选择下列 3 种信号处理方式中的一种：

1. 执行默认操作
2. 忽略该信号
3. 捕获该信号，但是通过信号句柄来调用自定义的处理函数

信号可能被阻塞。进程中的每个线程拥有独立的信号掩码，用来表示本线程的信号被阻塞。线程通过 pthread_sigmask 来设置它的信号掩码。单线程程序可以用 sigprocmask
来操作信号掩码。在多线程程序中，所有线程处理一个指定信号的默认行为都是一样的。