use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Dont exist Room. You need to create it")]
    DontExistRoom,
    #[error("Dont exist Info about this Device")]
    DontExistInfo,
    #[error("Dont exist Room. You need to create it")]
    DontExistDevice,
}
