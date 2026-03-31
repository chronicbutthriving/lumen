use std::sync::Arc;

pub struct Context {
    storage: Arc<dyn lumen_storage_db::dbs::StorageStore>,
}

impl Context {
    pub fn new(storage: impl lumen_storage_db::dbs::StorageStore) -> Self {
        Self {
            storage: Arc::new(storage),
        }
    }

    pub fn storage(&self) -> Arc<dyn lumen_storage_db::dbs::StorageStore> {
        self.storage.clone()
    }
}
