use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDataReferenceEntry {
    asset_id: String,
    agreement_id: String,
    transfer_process_id: String,
    provider_id: String,
    contract_negotiation_id: Option<String>,
    created_at: i64,
}

impl EndpointDataReferenceEntry {
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn agreement_id(&self) -> &str {
        &self.agreement_id
    }

    pub fn transfer_process_id(&self) -> &str {
        &self.transfer_process_id
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn contract_negotiation_id(&self) -> Option<&String> {
        self.contract_negotiation_id.as_ref()
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }
}
