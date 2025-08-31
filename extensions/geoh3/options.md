
Options
1. generated column and B-tree index
   1. pros
      1. minimal extension surface area
      2. planner friendly (uses normal B-tree)
      3. easy to add multi-resolution: h3_7, h3_9, h3_11 columns + indexes, pick res by radius.
   2. cons
      1. for very large areas -> many cells emitted
2. external virtual table as sidecar index

## Option 1: Generated Col and B-Tree index

```sql
CREATE TABLE places(
    id INTEGER PRIMARY KEY,
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    -- h3 cell at res=9
    h3_9 INTEGER GENERATED ALWAYS AS (h3_cell(lat, lon, 9)) STORED
);

CREATE INDEX places_h3_9_idx ON places(h3_9);
```