use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub enum Nav {
    #[default]
    ConnectorsList,
    AssetsList,
}

impl FromStr for Nav {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
