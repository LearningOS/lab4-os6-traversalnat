ch6:
    root inode 指向根目录的 disk node， 丢失会导致无法找到根目录。但根据其初始化设置，依然可以获取记录根目录的 disk node, 从而恢复 root inode。

ch7:
    1. pipe 一般在 shell 中使用，将一个程序的输出通过管道定向到另一个程序的标准输入
    2. 多进程通信可以使用 消息队列，进程将信息传递给内核的消息队列，由内核负责通知等待消息的进程；也可以使用共享内存等，但需要配合内存锁；也可以使用Socket 等，通过标准的Socket 进行点到点通信。
