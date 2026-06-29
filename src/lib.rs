use rltbl_db::db_value::DbRow;

pub trait ConvertDbRow {
    fn into_db_row(&self) -> DbRow;
    fn from_db_row(value: DbRow) -> Self;
}
