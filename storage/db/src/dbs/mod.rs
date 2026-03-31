//! Database access implementations for the application.

use std::collections::HashMap;

use lumen_uuid_kinds::ObjectUuid;

use crate::{models::ObjectModel, repos::ObjectStore};

pub trait StorageStore: ObjectStore + Send + Sync + 'static {}

pub struct MockStore {
    pub objects: HashMap<ObjectUuid, ObjectModel>,
}

impl MockStore {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
}

impl<T> StorageStore for T where T: ObjectStore + Send + Sync + 'static {}
