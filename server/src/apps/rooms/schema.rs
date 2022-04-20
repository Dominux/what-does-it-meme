table! {
    rooms {
        id -> Uuid,
        state -> Text,
        current_round_id -> Nullable<Uuid>,
        expiration_timestamp -> Timestamp,
    }
}
