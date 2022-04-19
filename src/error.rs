#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid compression level `{0}`")]
    InvalidCompressionLevel(String),

    #[error("Invalid compression format `{0}`")]
    InvalidCompressionFormat(String),

    #[error("Missing compression format argument")]
    MissingCompressionFormat,

    #[error("I/O Error")]
    Io(#[from] std::io::Error),

    #[error("Walk Dir Error")]
    WalkDir(#[from] walkdir::Error),

    #[error("I18n Embed Error:")]
    I18nEmbed(#[from] i18n_embed::I18nEmbedError),

    #[error("Persist Temporary File Error")]
    Persist(#[from] tempfile::PersistError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
