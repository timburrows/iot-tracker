use std::ops::AddAssign;

use entities::device;
use rand::Rng;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EntityTrait;
use time::{Date, Duration, Month, OffsetDateTime, Time};

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
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::InteractionId)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::Timestamp)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    // No SeaORM ColumnDef for more accurate data formats like Point, geog/geom with PostGIS etc
                    // Implement custom type here?
                    .col(
                        ColumnDef::new(DeviceInteractions::Longitude)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::Latitude)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceInteractions::DeviceId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-device_id")
                            .from(DeviceInteractions::Table, DeviceInteractions::DeviceId)
                            .to(Device::Table, Device::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed data
        // Create interactions using random lat/lon values for each device,
        // at intervals of 15 minutes for 7 days
        let devices: Vec<device::Model> =
            device::Entity::find().all(manager.get_connection()).await?;

        for dev in devices {
            let mut dt = OffsetDateTime::new_utc(
                Date::from_calendar_date(2024, Month::January, 1).unwrap(),
                Time::from_hms(00, 00, 0).unwrap(),
            );

            while dt.day() <= 7 {
                // simulate the device failing to submit data with a 30% frequency
                if rand::thread_rng().gen_range(0.0..1.0) < 0.8 {
                    let (lat, lon) = {
                        let mut rng = rand::thread_rng();
                        (rng.gen_range(-90.0..90.0), rng.gen_range(-180.0..180.0))
                    };

                    let insert = Query::insert()
                        .into_table(DeviceInteractions::Table)
                        .columns([
                            // Assuming that one interaction always has one unique interaction_id
                            // e.g no device would interact more than once with the same interaction_id
                            // Leave commented so that InteractionId auto increments
                            // DeviceInteractions::InteractionId,
                            DeviceInteractions::Timestamp,
                            DeviceInteractions::Latitude,
                            DeviceInteractions::Longitude,
                            DeviceInteractions::DeviceId,
                        ])
                        .values_panic([
                            // interaction_id.into(),
                            dt.into(),
                            lat.into(),
                            lon.into(),
                            dev.id.into(),
                        ])
                        .to_owned();

                    // todo: should bulk insert here
                    manager.exec_stmt(insert).await?;

                    dt.add_assign(Duration::minutes(15));
                }
            }
        }

        Ok(())
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
    DeviceId,
}
