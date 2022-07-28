root inode 指向根目录的 disk node， 丢失会导致无法找到根目录。但根据其初始化设置，依然可以获取记录根目录的 disk node, 从而恢复 root inode。
