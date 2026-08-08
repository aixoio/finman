
CREATE TABLE items (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    comment TEXT,
    type TEXT NOT NULL,
    target_cents INTEGER NOT NULL,
    current_cents INTEGER NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_items_archived ON items (archived);
