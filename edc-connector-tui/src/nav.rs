use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, Clone, Default)]
pub enum Nav {
    #[default]
    ConnectorsList,
    AssetsList,
}

impl FromStr for Nav {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "connectors" => Ok(Nav::ConnectorsList),
            "assets" => Ok(Nav::AssetsList),
            _ => bail!("Command {} not recognized", s),
        }
    }
}
