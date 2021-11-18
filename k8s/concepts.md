# docker (容器化技术核心概念)

## 进程隔离 而非 系统隔离

 容器化技术

 用户体验: 与 启动虚拟机系统 类型, 但是 比虚拟机启动快,占用资源比 虚拟机少

 应用进程: 共享 宿主系统 原有资源 (系统内核, 硬件资源), 在 进程 层级 实现 用户与 应用资源的隔离
 
 操作系统(linux/UNIX):  容器 实际依旧是 本系统下的 一到多个子进程 (只是在 相关 namespace 层隔离)  

#### 总结:

 Docker通过 `Namespace` 实现进程隔离

 通过 `linux` 系统函数 `clone()` 在创建新进程的同时创建 `namespace`。

```c
int clone(int (*child_func)(void *), void *child_stack, int flags, void *arg);
```

### namespace 隔离的类型:

- UTS namespace 

  UTS(UNIX Time-sharing System)namespace提供了主机名与域名的隔离，这样每个docker容器就可以拥有独立的主机名和域名了，在网络上可以被视为一个独立的节点，而非宿主机上的一个进程。

- IPC namespace

  进程间通信(Inter-Process Communication，IPC)涉及的IPC资源包括常见的信号量(`kill -l`)、消息队列和共享内存。在同一个IPC namespace下的进程彼此可见，不同IPC namespace下的进程则互相不可见。

- PID namespace

  PID namespace隔离非常实用，它对进程PID重新标号，即两个不同namespace下的进程可以有相同的PID。每个PID namespace都有自己的计数程序。内核为所有的PID namespace维护了一个树状结构，最顶层的是系统初始时创建的，被称为root namespace，它创建的心PID namespace被称为child namespace(树的子节点)。
通过这种方式，不同的PID namespace会形成一个层级体系。所属的父节点可以看到子节点中的进程，并可以通过信号等方式对子节点中的进程产生影响。反过来，子节点却不能看到父节点PID namespace中的任何内容。

> ### 实验

说明 : 

的系统调用是`unshare()`，它与`clone()`很像，不同的是，`unshare()`运行在原先的进程上，不需要启动一个新进程。

```shell
$ ps -ef #unshare 之前
$ sudo unshare --fork --pid --mount-proc bash
$ ps -ef #unshare 之后
```