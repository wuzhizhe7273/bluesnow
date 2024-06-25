pub use sea_orm_migration::prelude::*;

mod m0_1_create_table;
mod utils;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m0_1_create_table::Migration)]
    }
}
