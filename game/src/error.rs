pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("bag is at max capacity: {0}")]
    BagMax(u16),
    #[error(
        "item is at max capacity. {cap_needed} empty spots are required but only {remain} left"
    )]
    ItemMax { cap_needed: u64, remain: u64 },
    #[error(transparent)]
    DBError(#[from] mongodb::error::Error),
}
