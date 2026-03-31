use lumen_storage_api::StorageApi;

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn api() -> dropshot::ApiDescription<Context> {
    lumen_storage_api::storage_api_mod::api_description::<StorageApiImpl>()
        .expect("registered storage server endpoints")
}

enum StorageApiImpl {}

impl StorageApi for StorageApiImpl {
    type Context = Context;

    async fn ping(
        _rqctx: dropshot::RequestContext<Self::Context>,
    ) -> Result<dropshot::HttpResponseOk<lumen_storage_types_versions::latest::system::Ping>, dropshot::HttpError> {
        Ok(dropshot::HttpResponseOk(lumen_storage_types_versions::latest::system::Ping {
            status: lumen_storage_types_versions::latest::system::PingStatus::Ok,
        }))
    }
}
