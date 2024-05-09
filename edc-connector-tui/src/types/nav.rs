use std::str::FromStr;

use anyhow::bail;
use enum_ordinalize::Ordinalize;
use strum::{EnumString, VariantNames};

#[derive(Debug, Clone, Default)]
pub enum Nav {
    #[default]
    ConnectorsList,
    AssetsList,
    PoliciesList,
    ContractDefinitionsList,
}

impl FromStr for Nav {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "connectors" => Ok(Nav::ConnectorsList),
            "assets" => Ok(Nav::AssetsList),
            "policies" => Ok(Nav::PoliciesList),
            _ => bail!("Command {} not recognized", s),
        }
    }
}

#[derive(Debug, Default, Ordinalize, Clone, EnumString, VariantNames)]
#[repr(usize)]
pub enum Menu {
    #[default]
    Connectors,
    Assets,
    Policies,
    ContractDefinitions,
}

impl Menu {
    pub fn names() -> Vec<String> {
        <Menu as VariantNames>::VARIANTS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

impl Into<Menu> for Nav {
    fn into(self) -> Menu {
        match self {
            Nav::ConnectorsList => Menu::Connectors,
            Nav::AssetsList => Menu::Assets,
            Nav::PoliciesList => Menu::Policies,
            Nav::ContractDefinitionsList => Menu::ContractDefinitions,
        }
    }
}

impl Into<Nav> for Menu {
    fn into(self) -> Nav {
        match self {
            Menu::Connectors => Nav::ConnectorsList,
            Menu::Assets => Nav::AssetsList,
            Menu::Policies => Nav::PoliciesList,
            Menu::ContractDefinitions => Nav::ContractDefinitionsList,
        }
    }
}
