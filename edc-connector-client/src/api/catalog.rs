use crate::{
    client::EdcConnectorClientInternal,
    types::{
        catalog::{Catalog, CatalogRequest, Dataset, DatasetRequest},
        context::{WithContext, WithContextRef},
    },
    EdcResult,
};

pub struct CatalogApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> CatalogApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> CatalogApi<'a> {
        CatalogApi(client)
    }

    pub async fn request(&self, request: &CatalogRequest) -> EdcResult<Catalog> {
        let url = format!("{}/v2/catalog/request", self.0.management_url);
        self.0
            .post::<_, WithContext<Catalog>>(url, &WithContextRef::default_context(request))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn dataset(&self, request: &DatasetRequest) -> EdcResult<Dataset> {
        let url = format!("{}/v2/catalog/dataset/request", self.0.management_url);
        self.0
            .post::<_, WithContext<Dataset>>(url, &WithContextRef::default_context(request))
            .await
            .map(|ctx| ctx.inner)
    }
}
