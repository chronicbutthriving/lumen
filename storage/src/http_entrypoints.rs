use dropshot::{
    ApiDescription, HttpError, HttpResponseOk, Path, RequestContext,
};
use lumen_common::{api::external::error::Error, db::PaginationParams};
use lumen_storage_api::StorageApi;
use lumen_storage_db::repos::{ObjectFilter, ObjectStore};
use lumen_storage_types_versions::latest;

use crate::context::Context;

pub type StorageApiDescription = ApiDescription<Context>;

pub fn api() -> StorageApiDescription {
    lumen_storage_api::storage_api_mod::api_description::<StorageApiImpl>()
        .expect("registered storage server endpoints")
}

pub enum StorageApiImpl {}

impl StorageApi for StorageApiImpl {
    type Context = super::context::Context;

    async fn get_object(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<latest::object::GetObjectParams>,
    ) -> Result<HttpResponseOk<latest::object::StorageObject>, HttpError> {
        let storage = rqctx.context().storage();
        let path_params = path_params.into_inner();

        let result = ObjectStore::must_get(storage.as_ref(), path_params.id)
            .await
            .map_err(Error::from)?;

        Ok(HttpResponseOk(result.into()))
    }

    async fn list_objects(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Vec<latest::object::StorageObject>>, HttpError>
    {
        let storage = rqctx.context().storage();

        let result = ObjectStore::list(
            storage.as_ref(),
            ObjectFilter::new(),
            PaginationParams::default(),
        )
        .await
        .map_err(Error::from)?;

        Ok(HttpResponseOk(result.iter().map(|o| o.to_owned().into()).collect()))
    }

    async fn ping(
        _rqctx: dropshot::RequestContext<Self::Context>,
    ) -> Result<
        dropshot::HttpResponseOk<
            lumen_storage_types_versions::latest::system::Ping,
        >,
        dropshot::HttpError,
    > {
        Ok(dropshot::HttpResponseOk(
            lumen_storage_types_versions::latest::system::Ping {
                status:
                    lumen_storage_types_versions::latest::system::PingStatus::Ok,
            },
        ))
    }
}
