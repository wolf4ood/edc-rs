use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::BuilderError;

use super::{policy::Policy, query::Query, Protocol};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Catalog {
    #[serde(rename = "dataset", alias = "dcat:dataset")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    datasets: Vec<Dataset>,
}

impl Catalog {
    pub fn datasets(&self) -> &[Dataset] {
        &self.datasets
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Dataset {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "hasPolicy", alias = "odrl:hasPolicy")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    offers: Vec<Policy>,
}

impl Dataset {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn offers(&self) -> &[Policy] {
        &self.offers
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogRequest {
    counter_party_address: String,
    protocol: Protocol,
    counter_party_id: Option<String>,
    query_spec: Query,
}

impl CatalogRequest {
    pub fn builder() -> CatalogRequestBuilder {
        CatalogRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CatalogRequestBuilder {
    protocol: Protocol,
    counter_party_address: Option<String>,
    counter_party_id: Option<String>,
    query_spec: Query,
}

impl CatalogRequestBuilder {
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = Protocol::new(protocol);
        self
    }

    pub fn counter_party_address(mut self, counter_party_address: &str) -> Self {
        self.counter_party_address = Some(counter_party_address.to_string());
        self
    }

    pub fn counter_party_id(mut self, counter_party_id: &str) -> Self {
        self.counter_party_id = Some(counter_party_id.to_string());
        self
    }

    pub fn query_spec(mut self, query_spec: Query) -> Self {
        self.query_spec = query_spec;
        self
    }

    pub fn build(self) -> Result<CatalogRequest, BuilderError> {
        Ok(CatalogRequest {
            counter_party_address: self
                .counter_party_address
                .ok_or_else(|| BuilderError::missing_property("counter_party_address"))?,
            protocol: self.protocol,
            counter_party_id: self.counter_party_id,
            query_spec: self.query_spec,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetRequest {
    #[serde(rename = "@id")]
    id: String,
    counter_party_address: String,
    protocol: Protocol,
}

impl DatasetRequest {
    pub fn builder() -> DatasetRequestBuilder {
        DatasetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DatasetRequestBuilder {
    id: Option<String>,
    protocol: Protocol,
    counter_party_address: Option<String>,
}

impl DatasetRequestBuilder {
    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = Protocol::new(protocol);
        self
    }

    pub fn counter_party_address(mut self, counter_party_address: &str) -> Self {
        self.counter_party_address = Some(counter_party_address.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn build(self) -> Result<DatasetRequest, BuilderError> {
        Ok(DatasetRequest {
            id: self
                .id
                .ok_or_else(|| BuilderError::missing_property("id"))?,
            counter_party_address: self
                .counter_party_address
                .ok_or_else(|| BuilderError::missing_property("counter_party_address"))?,
            protocol: self.protocol,
        })
    }
}
