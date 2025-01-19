use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Sessions::Uuid).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Sessions::Data).binary().not_null())
                    .col(ColumnDef::new(Sessions::Expiry).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Sessions::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub(crate) enum Sessions {
    Table,
    Uuid,
    Data,
    Expiry,
}
