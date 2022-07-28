mod inode;
mod stdio;

use crate::mm::UserBuffer;

/// The common abstraction of all IO resources
pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn stat(&self) -> Stat {
        // Default Stat: no meaning
        Stat::default()
    }
}

pub use inode::{list_apps, open_file, OSInode, OpenFlags, link_at, unlink_at};
pub use stdio::{Stdin, Stdout};
pub use easy_fs::{EasyFileSystem, Inode, Stat, StatMode};
