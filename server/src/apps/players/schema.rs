table! {
    players {
        id -> Uuid,
        name -> Text,
        room_id -> Uuid,
        memes_in_hand -> Array<Text>,
    }
}
