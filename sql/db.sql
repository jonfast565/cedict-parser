CREATE TABLE ce_line (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    simplified TEXT NOT NULL,
    traditional TEXT NOT NULL
);

CREATE TABLE pinyin (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pinyin TEXT NOT NULL UNIQUE
);

CREATE TABLE definitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    definition TEXT NOT NULL UNIQUE
);

CREATE TABLE ce_line_pinyin (
    ce_line_id INTEGER NOT NULL,
    pinyin_id INTEGER NOT NULL,
    FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
    FOREIGN KEY (pinyin_id) REFERENCES pinyin(id),
    PRIMARY KEY (ce_line_id, pinyin_id)
);

CREATE TABLE ce_line_definitions (
    ce_line_id INTEGER NOT NULL,
    definition_id INTEGER NOT NULL,
    FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
    FOREIGN KEY (definition_id) REFERENCES definitions(id),
    PRIMARY KEY (ce_line_id, definition_id)
);
