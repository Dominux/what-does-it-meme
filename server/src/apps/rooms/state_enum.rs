use diesel::{
    pg::Pg,
    sql_types::Text,
    types::{FromSql, IsNull, ToSql},
};
use serde::{Serialize, Deserialize};

// diesel-enum and diesel-enum-derive didn't work for me for some reason
// so it created by following the code example:
// https://spectrum.chat/rust/general/storing-rust-enums-in-postgres-with-diesel~be6a432e-57b6-4313-b82d-367fbf89312d

#[derive(Serialize, Deserialize, Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum RoomState {
    NotStarted,
    Started,
    Ended,
}

impl ToSql<Text, Pg> for RoomState {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            RoomState::NotStarted => out.write_all(b"not_started")?,
            RoomState::Started => out.write_all(b"started")?,
            RoomState::Ended => out.write_all(b"ended")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for RoomState {
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        match not_none!(bytes) {
            b"not_started" => Ok(RoomState::NotStarted),
            b"started" => Ok(RoomState::Started),
            b"ended" => Ok(RoomState::Ended),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}

