table! {
    rounds {
        id -> Uuid,
        room_id -> Uuid,
        number -> SmallInt,
        state -> Text,
        situation -> Nullable<Text>,
        situation_creater_id -> Uuid,
    }
}
