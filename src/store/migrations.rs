use super::StoreError;
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

// MIGRATIONS
// ================================================================================================

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::new(vec![M::up(include_str!("store.sql")),]);
}

// PUBLIC FUNCTIONS
// ================================================================================================

pub(crate) fn update_to_latest(conn: &mut Connection) -> Result<(), StoreError> {
    Ok(MIGRATIONS.to_latest(conn)?)
}
