#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid compression level `{0}`")]
    InvalidCompressionLevel(String),

    #[error("Invalid compression format `{0}`")]
    InvalidCompressionFormat(String),

    #[error("Missing compression format argument")]
    MissingCompressionFormat,

    #[error("Walk Dir Error")]
    WalkDir(#[from] walkdir::Error),

    #[error("I18n Embed Error:")]
    I18nEmbed(#[from] i18n_embed::I18nEmbedError),

    #[error("Persist Temporary File Error")]
    Persist(#[from] tempfile::PersistError),

    #[error("Could not create archive file")]
    CreateArchiveFile(#[source] std::io::Error),

    #[error("Could not remove old archive file")]
    RemoveOldArchive(#[source] std::io::Error),

    #[error("Tar Packer Error")]
    TarPacker(#[source] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
