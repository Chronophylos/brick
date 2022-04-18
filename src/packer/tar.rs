use std::{io::Write, path::Path};

use log::debug;
use tar::Builder;

use crate::error::Result;

use super::Packer;

pub struct TarPacker<W>
where
    W: Write,
{
    tar: Builder<W>,
}

impl<W> TarPacker<W>
where
    W: Write,
{
    pub fn new(file: W) -> Result<Self> {
        Ok(Self {
            tar: Builder::new(file),
        })
    }
}

impl<W> Packer for TarPacker<W>
where
    W: Write,
{
    fn add_dir(&mut self, path: &Path) -> Result<()> {
        debug!("Adding directory `{}` to archive", path.display());

        self.tar.append_dir_all(path, path)?;

        Ok(())
    }

    fn add_file(&mut self, path: &Path) -> Result<()> {
        debug!("Adding file `{}` to archive", path.display());

        self.tar.append_path(path)?;

        Ok(())
    }

    fn finish(self) -> Result<()> {
        debug!("Finishing tar packer");

        self.tar.into_inner()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        error::Error,
        fs::{self, read_to_string, File},
    };

    use pretty_assertions::assert_eq;
    use tar::Archive;
    use tempfile::{tempdir, NamedTempFile};

    use super::*;

    /// Create test logger instance
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    /// test packer with a single file
    #[test]
    fn pack_single_file() -> Result<(), Box<dyn Error>> {
        init();
        const FILE_NAME: &str = "some-file.txt";
        const FILE_CONTENT: &[u8] = b"some test text";

        // setup working directory

        let working_directory = tempdir()?;
        env::set_current_dir(&working_directory)?;

        // create test file

        let file_path = working_directory.path().join(FILE_NAME);
        let mut file = File::create(&file_path)?;
        file.write_all(FILE_CONTENT)?;

        // create archive file

        let archive_write_handle = NamedTempFile::new_in(&working_directory)?;
        let archive_read_handle = archive_write_handle.reopen()?;

        // run packer

        let mut packer = TarPacker::new(archive_write_handle)?;
        packer.add_path(file_path.strip_prefix(&working_directory)?)?;
        packer.finish()?;

        // unpack archive with tar

        let out_dir = tempdir()?;

        let mut tar = Archive::new(archive_read_handle);
        tar.unpack(&out_dir)?;

        // run test

        let content = read_to_string(out_dir.path().join(FILE_NAME))?;
        assert_eq!(content.as_bytes(), FILE_CONTENT);

        Ok(())
    }

    /// test packer with a directory
    #[test]
    fn pack_directory() -> Result<(), Box<dyn Error>> {
        init();
        const TOP_FILE_NAME: &str = "some-file.txt";
        const TOP_FILE_CONTENT: &[u8] = b"some test text";
        const DIRECTORY_NAME: &str = "directory";
        const INNER_FILE_NAME: &str = "inner-file.txt";
        const INNER_FILE_CONTENT: &[u8] = b"some different test text";

        // setup working directory

        let working_directory = tempdir()?;
        env::set_current_dir(&working_directory)?;

        // create top test file

        let top_file_path = working_directory.path().join(TOP_FILE_NAME);
        let mut top_file = File::create(&top_file_path)?;
        top_file.write_all(TOP_FILE_CONTENT)?;

        // create directory

        let directory = working_directory.path().join(DIRECTORY_NAME);
        fs::create_dir(&directory)?;

        // create inner test file

        let inner_file_path = directory.join(INNER_FILE_NAME);
        let mut inner_file = File::create(&inner_file_path)?;
        inner_file.write_all(INNER_FILE_CONTENT)?;

        // create archive file

        let archive_write_handle = NamedTempFile::new_in(&working_directory)?;
        let archive_read_handle = archive_write_handle.reopen()?;

        // run packer

        let mut packer = TarPacker::new(archive_write_handle)?;
        packer.add_path(top_file_path.strip_prefix(&working_directory)?)?;
        packer.add_path(directory.strip_prefix(&working_directory)?)?;
        packer.finish()?;

        // unpack archive with tar

        let out_dir = tempdir()?;

        let mut tar = Archive::new(archive_read_handle);
        tar.unpack(&out_dir)?;

        // run tests

        let top_content = read_to_string(out_dir.path().join(TOP_FILE_NAME))?;
        assert_eq!(top_content.as_bytes(), TOP_FILE_CONTENT);

        let inner_content =
            read_to_string(out_dir.path().join(DIRECTORY_NAME).join(INNER_FILE_NAME))?;
        assert_eq!(inner_content.as_bytes(), INNER_FILE_CONTENT);

        Ok(())
    }
}
