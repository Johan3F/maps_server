// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    elements (id) {
        id -> Uuid,
        collection_id -> Nullable<Uuid>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    geometries (element_id) {
        element_id -> Uuid,
    }
}

diesel::table! {
    points (element_id) {
        element_id -> Uuid,
    }
}

diesel::table! {
    tracks (element_id) {
        element_id -> Uuid,
    }
}

diesel::joinable!(elements -> collections (collection_id));

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    elements,
    geometries,
    points,
    tracks,
);
