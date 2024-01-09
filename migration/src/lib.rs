pub use sea_orm_migration::prelude::*;

mod m20220107_000001_create_device_table;
mod m20220107_000002_create_device_interactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220107_000001_create_device_table::Migration),
            Box::new(m20220107_000002_create_device_interactions_table::Migration)
        ]
    }
}
