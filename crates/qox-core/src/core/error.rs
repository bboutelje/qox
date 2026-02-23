use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum QoxError {
    #[error(transparent)]
    Curve(#[from] CurveError),
    #[error(transparent)]
    Interpolation(#[from] InterpolationError),
}

#[derive(Debug, Error)]
pub enum InterpolationError {
    #[error("x and y must have same length")]
    LengthMismatch,
    #[error("need at least 2 points")]
    InsufficientPoints,
    #[error("values must be strictly increasing")]
    NotMonotonic,
}

#[derive(Debug, Error)]
pub enum CurveError {
    #[error("tenors and rates must have same length")]
    LengthMismatch,
    #[error(transparent)]
    Interpolation(#[from] InterpolationError),
}