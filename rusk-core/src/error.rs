pub enum RuskError {
    TaskExecutionError(String),
    PartitionError(String),
    SerializationError(String),
    NetworkError(String),
    StorageError(String),
}

impl From<std::io::Error> for RuskError
impl std::fmt::Display for RuskError
impl std::error::Error for RuskError
