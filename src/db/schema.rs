// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "geometry"))]
    pub struct Geometry;
}

diesel::table! {
    use postgis_diesel::sql_types::*;

    collections (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;

    elements (id) {
        id -> Uuid,
        collection_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;

    geometries (element_id) {
        element_id -> Uuid,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use super::sql_types::Geometry;

    points (element_id) {
        element_id -> Uuid,
        point -> Geometry,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;

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
    spatial_ref_sys,
    tracks,
);
