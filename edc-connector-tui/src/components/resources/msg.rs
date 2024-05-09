use crate::components::table::msg::TableMsg;

use super::resource::msg::ResourceMsg;

#[derive(Debug)]
pub enum ResourcesMsg<T> {
    ResourceSelected(T),
    Back,
    TableEvent(TableMsg<Box<ResourcesMsg<T>>>),
    ResourceMsg(ResourceMsg),
    ResourcesFetched(Vec<T>),
}
