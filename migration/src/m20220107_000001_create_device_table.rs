use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Device::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Device::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Device::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Seed data
        let insert = Query::insert()
            .into_table(Device::Table)
            .columns([Device::Name])
            .values_panic(["PlayStation".into()])
            .values_panic(["PS One".into()])
            .values_panic(["PlayStation 2".into()])
            .values_panic(["PlayStation 3".into()])
            .values_panic(["PlayStation 4".into()])
            .values_panic(["PlayStation 4 Pro".into()])
            .values_panic(["PlayStation 5".into()])
            .values_panic(["Xbox".into()])
            .values_panic(["Xbox 360".into()])
            .values_panic(["Xbox One".into()])
            .values_panic(["Xbox Series S".into()])
            .values_panic(["Xbox Series X".into()])
            .values_panic(["PC".into()])
            .values_panic(["Mac".into()])
            .values_panic(["Raspberi Pi 1".into()])
            .values_panic(["Raspberi Pi 2".into()])
            .values_panic(["Raspberi Pi 3".into()])
            .values_panic(["Raspberi Pi 4".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Device::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Device {
    Table,
    Id,
    Name,
}
