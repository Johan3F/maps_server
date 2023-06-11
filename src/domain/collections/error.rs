#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error: {source:}")]
    Unknown {
        #[from]
        source: anyhow::Error,
    },
    #[error("collection '{id:}' not found")]
    NotFound { id: uuid::Uuid },
}

impl From<deadpool::managed::PoolError<deadpool_diesel::Error>> for Error {
    fn from(inner: deadpool::managed::PoolError<deadpool_diesel::Error>) -> Self {
        Error::Unknown {
            source: inner.into(),
        }
    }
}

impl From<deadpool_sync::InteractError> for Error {
    fn from(inner: deadpool_sync::InteractError) -> Self {
        Error::Unknown {
            source: anyhow::anyhow!(inner.to_string()),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(inner: diesel::result::Error) -> Self {
        Error::Unknown {
            source: inner.into(),
        }
    }
}
