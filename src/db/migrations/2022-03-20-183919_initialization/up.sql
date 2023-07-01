CREATE TABLE collections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
);

CREATE TABLE elements (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id uuid NOT NULL,
    name TEXT NOT NULL,
    description TEXT,

    CONSTRAINT fk_elements_collections FOREIGN KEY(collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

CREATE TABLE points (
    element_id uuid PRIMARY KEY NOT NULL,
    point geometry(Point,4326) NOT NULL,

    CONSTRAINT fk_points_elements FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE tracks (
    element_id uuid PRIMARY KEY NOT NULL,

    CONSTRAINT fk_tracks_elements FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE geometries (
    element_id uuid PRIMARY KEY NOT NULL,

    CONSTRAINT fk_geometries_elements FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);