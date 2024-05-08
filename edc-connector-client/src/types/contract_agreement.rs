use serde::Deserialize;

use super::policy::Policy;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAgreement {
    #[serde(rename = "@id")]
    id: String,
    contract_signing_date: i64,
    asset_id: String,
    consumer_id: String,
    provider_id: String,
    policy: Policy,
}

impl ContractAgreement {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn contract_signing_date(&self) -> i64 {
        self.contract_signing_date
    }

    pub fn consumer_id(&self) -> &str {
        &self.consumer_id
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn policy(&self) -> &Policy {
        &self.policy
    }
}
