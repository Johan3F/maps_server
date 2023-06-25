CREATE TABLE collections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
);

CREATE TABLE elements (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id uuid,
    name TEXT NOT NULL,
    description TEXT,

    CONSTRAINT fk_collections FOREIGN KEY(collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

CREATE TABLE points (
    element_id uuid PRIMARY KEY,

    CONSTRAINT fk_element FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE tracks (
    element_id uuid PRIMARY KEY,

    CONSTRAINT fk_element FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE geometries (
    element_id uuid PRIMARY KEY,

    CONSTRAINT fk_element FOREIGN KEY(element_id) REFERENCES elements(id) ON DELETE CASCADE
);