use crate::{
    client::EdcConnectorClientInternal,
    types::{
        asset::{Asset, NewAsset},
        context::{WithContext, WithContextRef},
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct AssetApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> AssetApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> AssetApi<'a> {
        AssetApi(client)
    }

    pub async fn create(&self, asset: &NewAsset) -> EdcResult<IdResponse<String>> {
        let url = format!("{}/v3/assets", self.0.management_url);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::default_context(asset),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Asset> {
        let url = format!("{}/v3/assets/{}", self.0.management_url, id);
        self.0
            .get::<WithContext<Asset>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, asset: &Asset) -> EdcResult<()> {
        let url = format!("{}/v3/assets", self.0.management_url);
        self.0
            .put(url, &WithContextRef::default_context(asset))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<Asset>> {
        let url = format!("{}/v3/assets/request", self.0.management_url);
        self.0
            .post::<_, Vec<WithContext<Asset>>>(url, &WithContextRef::default_context(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = format!("{}/v3/assets/{}", self.0.management_url, id);
        self.0.del(url).await
    }
}
