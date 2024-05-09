use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        contract_agreement::ContractAgreement,
        query::Query,
    },
    EdcResult,
};

pub struct ContractAgreementApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractAgreementApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractAgreementApi<'a> {
        ContractAgreementApi(client)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractAgreement> {
        let url = format!("{}/v3/contractagreements/{}", self.0.management_url, id);
        self.0
            .get::<WithContext<ContractAgreement>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractAgreement>> {
        let url = format!("{}/v3/contractagreements/request", self.0.management_url);
        self.0
            .post::<_, Vec<WithContext<ContractAgreement>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}
