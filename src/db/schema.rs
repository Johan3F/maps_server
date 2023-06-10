use diesel::prelude::*;

table! {
    collections (id) {
        id -> Uuid,
        name -> Text,
    }
}
