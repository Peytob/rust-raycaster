pub mod tile;
pub mod tilemap;
pub mod repository;

pub type ResourceId = u32;

pub fn is_exists_resource(resource_id: ResourceId) -> bool {
    resource_id > 0
}
