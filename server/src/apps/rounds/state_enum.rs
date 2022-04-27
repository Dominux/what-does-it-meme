use diesel::{
    pg::Pg,
    sql_types::Text,
    types::{FromSql, IsNull, ToSql},
};
use serde::{Deserialize, Serialize};

// diesel-enum and diesel-enum-derive didn't work for me for some reason
// so it created by following the code example:
// https://spectrum.chat/rust/general/storing-rust-enums-in-postgres-with-diesel~be6a432e-57b6-4313-b82d-367fbf89312d

#[derive(Serialize, Deserialize, Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum RoundState {
    SituationCreation,
    ChoosingMemes,
    Voting,
    ShowingResults,
    Ended,
}

impl ToSql<Text, Pg> for RoundState {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            RoundState::SituationCreation => out.write_all(b"situation_creation")?,
            RoundState::ChoosingMemes => out.write_all(b"choosing_memes")?,
            RoundState::Voting => out.write_all(b"voting")?,
            RoundState::ShowingResults => out.write_all(b"showing_results")?,
            RoundState::Ended => out.write_all(b"ended")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for RoundState {
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> diesel::deserialize::Result<Self> {
        match not_none!(bytes) {
            b"situation_creation" => Ok(RoundState::SituationCreation),
            b"choosing_memes" => Ok(RoundState::ChoosingMemes),
            b"voting" => Ok(RoundState::Voting),
            b"showing_results" => Ok(RoundState::ShowingResults),
            b"ended" => Ok(RoundState::Ended),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
