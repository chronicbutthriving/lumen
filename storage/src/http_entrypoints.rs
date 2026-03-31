use dropshot::ApiDescription;
use lumen_storage_api::StorageApi;

use crate::context::Context;

pub type StorageApiDescription = ApiDescription<Context>;

pub fn api() -> StorageApiDescription {
    lumen_storage_api::storage_api_mod::api_description::<StorageApiImpl>()
        .expect("registered storage server endpoints")
}

pub enum StorageApiImpl {}

impl StorageApi for StorageApiImpl {
    type Context = super::context::Context;

    async fn ping(
        _rqctx: dropshot::RequestContext<Self::Context>,
    ) -> Result<dropshot::HttpResponseOk<lumen_storage_types_versions::latest::system::Ping>, dropshot::HttpError> {
        Ok(dropshot::HttpResponseOk(lumen_storage_types_versions::latest::system::Ping {
            status: lumen_storage_types_versions::latest::system::PingStatus::Ok,
        }))
    }
}
