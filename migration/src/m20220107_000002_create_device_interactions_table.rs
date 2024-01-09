use sea_orm_migration::prelude::*;

use crate::m20220107_000001_create_device_table::Device;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceInteractions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceInteractions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::InteractionId)
                            .integer()
                            .not_null()
                            .auto_increment()
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::Timestamp)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    // No SeaORM ColumnDef for more accurate data formats like Point, geog/geom with PostGIS etc
                    // Implement custom type here?
                    .col(
                        ColumnDef::new(DeviceInteractions::Longitude)
                            .float()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::Latitude)
                            .float()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::DeviceId)
                            .integer()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-device_id")
                            .from(DeviceInteractions::Table, DeviceInteractions::DeviceId)
                            .to(Device::Table, Device::Id)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceInteractions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeviceInteractions {
    Table,
    Id,
    InteractionId,
    Timestamp,
    Longitude,
    Latitude,
    DeviceId
}