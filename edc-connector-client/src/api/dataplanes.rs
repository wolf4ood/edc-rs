use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        dataplane::DataPlaneInstance,
        response::IdResponse,
    },
    EdcResult,
};

pub struct DataPlaneApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> DataPlaneApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> DataPlaneApi<'a> {
        DataPlaneApi(client)
    }

    pub async fn register(&self, data_plane: &DataPlaneInstance) -> EdcResult<IdResponse<String>> {
        let url = format!("{}/v2/dataplanes", self.0.management_url);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::default_context(data_plane),
            )
            .await
            .map(|ctx| ctx.inner)
    }
}
