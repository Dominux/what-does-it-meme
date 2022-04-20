table! {
    memes {
        id -> Uuid,
        round_id -> Uuid,
        player_id -> Uuid,
        voters_ids -> Array<Uuid>,
        link -> Text,
    }
}
