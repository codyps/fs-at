pub use std::io::{Error,Result};

#[cfg(unix)]
mod sys {
    pub struct Dir {
        fd: RawFd,
    }

    pub struct ReadDir {
        _x: (),
    }
}

#[cfg(windows)]
mod sys {
    pub struct Dir {
        fd: FileHandle,
    }

    pub struct ReadDir {
        _x: (),
    }
}

pub struct Dir {
    sys: sys::Dir,
}

impl Dir {
    /// Open a directory at a given path (absolute or relative to the current working directory)
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        unimplimented!();
    }

    /// Open a directory relative to this directory
    pub fn open_dir_at<P: AsRef<Path>>(&self, relative_path: P) -> Result<Self> {
        unimplimented!();
    }

    pub fn mkdir_at<P: AsRef<Path>>(&self, relative_path: P) -> Result<()> {
        unimplimented!();
    }

    pub fn rename_at<Ps: AsRef<Path>, Pd: AsRef<Path>>(&self, src_path: Ps, dst_dir: Dir, dst_path: Pd) -> Result<()> {
        unimplimented!();
    }

    /// Get an iterator over the elements of a directory
    pub fn read_dir(&self) -> Result<ReadDir> {
        unimplimented!();
    }
}

/// Iterator over entries within a directory
///
/// Provided by [`Dir::read_dir()`](TODO LINK)
pub struct ReadDir {
    inner: sys::ReadDir
}

trait FsAtFileExt {
    fn rename_at<Pd: AsRef<Path>>(&self, dst_dir: Dir, dst_path: Pd) -> Result<()>;
    fn link_at<Pd: AsRef<Path>>(&self, dst_dir: Dir, dst_path: Pd) -> Result<()>;

    /// Open a file relative to a directory
    ///
    /// TODO: OpenBuilder, various flags, etc.
    pub fn open_at<P: AsRef<Path>>(&self, dir: Dir, relative_path: P) -> Result<std::io::File> {
        unimplimented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
