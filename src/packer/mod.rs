pub mod tar;

use std::path::Path;

use walkdir::WalkDir;

use crate::error::Result;

pub trait Packer {
    /// Add a path to the packer
    ///
    /// If the path points to a file add the file (see [Packer::add_file])
    /// otherwise add a directory (see [Packer::add_dir])
    fn add_path(&mut self, path: &Path) -> Result<()> {
        if path.is_dir() {
            self.add_dir(path)
        } else {
            self.add_file(path)
        }
    }

    /// Recursively add a directory and all files within to the packer
    fn add_dir(&mut self, path: &Path) -> Result<()> {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            self.add_path(entry.path())?;
        }

        Ok(())
    }

    /// Add a file to the packer
    fn add_file(&mut self, path: &Path) -> Result<()>;

    /// Finish writing the archive and finish all outstanding operations
    fn finish(self) -> Result<()>;
}

pub trait Unpacker: Sized {
    /// Open an archive
    fn open(path: &Path) -> Result<Self>;

    /// Unpack the archive to the path
    fn unpack(self, path: &Path) -> Result<()>;
}
