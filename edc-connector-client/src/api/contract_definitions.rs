use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        contract_definition::{ContractDefinition, NewContractDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct ContractDefinitionApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractDefinitionApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractDefinitionApi<'a> {
        ContractDefinitionApi(client)
    }

    pub async fn create(
        &self,
        contract_definition: &NewContractDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = format!("{}/v2/contractdefinitions", self.0.management_url);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::default_context(contract_definition),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractDefinition> {
        let url = format!("{}/v2/contractdefinitions/{}", self.0.management_url, id);
        self.0
            .get::<WithContext<ContractDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, contract_definition: &ContractDefinition) -> EdcResult<()> {
        let url = format!("{}/v2/contractdefinitions", self.0.management_url);
        self.0
            .put(url, &WithContextRef::default_context(contract_definition))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractDefinition>> {
        let url = format!("{}/v2/contractdefinitions/request", self.0.management_url);
        self.0
            .post::<_, Vec<WithContext<ContractDefinition>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = format!("{}/v2/contractdefinitions/{}", self.0.management_url, id);
        self.0.del(url).await
    }
}
