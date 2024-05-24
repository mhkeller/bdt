use datafusion::error::DataFusionError;

pub mod utils;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    General(String),
    #[error("Data Fusion error: {0}")]
    DataFusion(#[from] DataFusionError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
pub enum FileFormat {
    Csv,
}
