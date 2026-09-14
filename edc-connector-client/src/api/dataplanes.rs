use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        dataplane::{DataPlaneInstance, DataPlaneRegistrationMessage},
    },
    EdcConnectorApiVersion, EdcResult,
};

const DATAPLANES_PATH: &str = "dataplanes";

pub struct DataPlaneApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> DataPlaneApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> DataPlaneApi<'a> {
        DataPlaneApi { client, version }
    }

    /// Registers a data plane, or updates it when one with the same id is
    /// already registered.
    pub async fn register(&self, registration: &DataPlaneRegistrationMessage) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH]);
        self.client.put(url, registration).await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH, id]);
        self.client.del(url).await
    }

    pub async fn list(&self) -> EdcResult<Vec<DataPlaneInstance>> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH]);
        self.client
            .get::<Vec<WithContext<DataPlaneInstance>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}
