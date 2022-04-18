pub mod error;
pub mod packer;

use std::{fmt, str::FromStr};

use error::Error;

#[derive(Debug, Clone, Copy)]
pub enum CompressionLevel {
    Auto,
    Numbered(u8),
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self::Auto
    }
}

impl FromStr for CompressionLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "0" => Ok(Self::Numbered(0)),
            "1" => Ok(Self::Numbered(1)),
            "2" => Ok(Self::Numbered(2)),
            "3" => Ok(Self::Numbered(3)),
            "4" => Ok(Self::Numbered(4)),
            "5" => Ok(Self::Numbered(5)),
            "6" => Ok(Self::Numbered(6)),
            "7" => Ok(Self::Numbered(7)),
            "8" => Ok(Self::Numbered(8)),
            "9" => Ok(Self::Numbered(9)),
            _ => Err(Error::InvalidCompressionLevel(s.to_owned())),
        }
    }
}

impl fmt::Display for CompressionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionLevel::Auto => write!(f, "auto"),
            CompressionLevel::Numbered(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArchiveFormat {
    Tar,
    Zip,
    GZip,
    Lzma,
}

impl FromStr for ArchiveFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tar" => Ok(Self::Tar),
            "zip" => Ok(Self::Zip),
            "gzip" => Ok(Self::GZip),
            "lzma" => Ok(Self::Lzma),
            _ => Err(Error::InvalidCompressionFormat(s.to_owned())),
        }
    }
}

impl fmt::Display for ArchiveFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchiveFormat::Tar => write!(f, "tar"),
            ArchiveFormat::Zip => write!(f, "zip"),
            ArchiveFormat::GZip => write!(f, "gzip"),
            ArchiveFormat::Lzma => write!(f, "lzma"),
        }
    }
}

impl ArchiveFormat {
    pub const fn all() -> &'static [&'static str] {
        &["tar", "zip", "gzip", "lzma"]
    }

    pub fn try_from_ext(ext: &str) -> Vec<ArchiveFormat> {
        match ext {
            "tar" => vec![Self::Tar],
            "tgz" => vec![Self::Tar, Self::GZip],
            "gz" => vec![Self::GZip],
            "xz" | "lz" | "lzma" => vec![Self::Lzma],
            "zip" => vec![Self::Zip],
            _ => Vec::new(),
        }
    }
}
