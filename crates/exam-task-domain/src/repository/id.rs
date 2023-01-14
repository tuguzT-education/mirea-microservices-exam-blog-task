use async_trait::async_trait;
use auto_impl::auto_impl;

use crate::model::{ErasedId, InternalError};

#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait IdRepository: Send + Sync {
    async fn create(&self) -> Result<ErasedId>;
}

pub type Result<T> = core::result::Result<T, InternalError>;
