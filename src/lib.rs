pub use std::io::{Error,Result};

#[cfg(unix)]
mod sys {
    pub struct Dir {
        fd: RawFd,
    }
}

#[cfg(windows)]
mod sys {
    pub struct Dir {
        fd: FileHandle,
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

    /// Open a file relative to this directory
    ///
    /// TODO: a bunch of crazy open mechanisms
    pub fn open_file_at<P: AsRef<Path>>(&self, relative_path: P) -> Result<std::io::File> {
        unimplimented!();
    }

    pub fn mkdir_at<P: AsRef<Path>>(&self, relative_path: P) -> Result<()> {
        unimplimented!();
    }

    pub fn rename_at<Ps: AsRef<Path>, Pd: AsRef<Path>>(&self, src_path: Ps, dst_dir: Dir, dst_path: Pd) -> Result<()> {
        unimplimented!();
    }
}

trait FsAtFileExt {
    fn rename_at<Pd: AsRef<Path>>(&self, dst_dir: Dir, dst_path: Pd) -> Result<()>;
    fn link_at<Pd: AsRef<Path>>(&self, dst_dir: Dir, dst_path: Pd) -> Result<()>;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
