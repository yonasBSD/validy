use crate::core::SpecificAsyncValidateAndParseWithContext;

pub struct Valid<T: SpecificAsyncValidateAndParseWithContext>(pub T);
#[cfg(feature = "axum_multipart")]
pub struct ValidMultipart<T: SpecificAsyncValidateAndParseWithContext>(pub T);
