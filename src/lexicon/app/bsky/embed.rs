use crate::lexicon::com::atproto::repo::{Blob, Link};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub image: Blob,
    pub alt: String,
}
