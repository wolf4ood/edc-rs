use crate::components::table::msg::TableMsg;

#[derive(Debug)]
pub enum ResourcesMsg<T> {
    ResourceSelected(T),
    Back,
    TableEvent(TableMsg<Box<ResourcesMsg<T>>>),
    ResourcesFetched(Vec<T>),
}
