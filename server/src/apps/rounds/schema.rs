table! {
    rounds {
        id -> Uuid,
        room_id -> Uuid,
        state -> Text,
        situation -> Nullable<Text>,
        situation_creator_id -> Uuid,
    }
}
