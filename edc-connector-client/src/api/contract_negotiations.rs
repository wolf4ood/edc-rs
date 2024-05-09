use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        contract_negotiation::{
            ContractNegotiation, ContractNegotiationState, ContractRequest, NegotiationState,
            TerminateNegotiation,
        },
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct ContractNegotiationApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractNegotiationApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractNegotiationApi<'a> {
        ContractNegotiationApi(client)
    }

    pub async fn initiate(
        &self,
        contract_request: &ContractRequest,
    ) -> EdcResult<IdResponse<String>> {
        let url = format!("{}/v3/contractnegotiations", self.0.management_url);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::odrl_context(contract_request),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractNegotiation> {
        let url = format!("{}/v3/contractnegotiations/{}", self.0.management_url, id);
        self.0
            .get::<WithContext<ContractNegotiation>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<ContractNegotiationState> {
        let url = format!("{}/v3/contractnegotiations/{}", self.0.management_url, id);
        self.0
            .get::<WithContext<NegotiationState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = format!(
            "{}/v3/contractnegotiations/{}/terminate",
            self.0.management_url, id
        );

        let request = TerminateNegotiation {
            id: id.to_string(),
            reason: reason.to_string(),
        };
        self.0
            .post_no_response(url, &WithContextRef::default_context(&request))
            .await
            .map(|_| ())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractNegotiation>> {
        let url = format!("{}/v3/contractnegotiations/request", self.0.management_url);
        self.0
            .post::<_, Vec<WithContext<ContractNegotiation>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}
