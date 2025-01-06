use sea_orm_migration::prelude::*;

use crate::m20250106_194018_users::Users;

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
                    .col(ColumnDef::new(Sessions::Id).big_integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Sessions::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Sessions::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Sessions::Expiry).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_users_id")
                            .from(Sessions::Table, Sessions::UserId)
                            .to(Users::Table, Users::Id),
                    )
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
    Id,
    Uuid,
    UserId,
    Expiry,
}
