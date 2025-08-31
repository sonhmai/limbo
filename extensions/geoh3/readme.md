# H3 Geospatial Index Extension

## Overview

This extension implements H3 geospatial indexing capabilities for Turso, enabling efficient spatial data storage, indexing, and querying. H3 is Uber's hierarchical geospatial indexing system that partitions the world into hexagonal cells at multiple resolutions.

Design decisions
1. Data storage
   - H3 cell indices stored as 64-bit integers (`H3Index`)
   - Support for all H3 resolutions (0-15)
2. H3 library h3o: pure Rust implementation

## Extension API

### Scalar Functions
- `h3_from_latlng(lat, lng, resolution)` - Convert coordinates to H3 cell
- `h3_to_latlng(h3_index)` - Convert H3 cell to center coordinates
- `h3_to_boundary(h3_index)` - Get cell boundary as polygon
- `h3_neighbors(h3_index)` - Get neighboring cells
- `h3_parent(h3_index, parent_res)` - Get parent cell at lower resolution
- `h3_children(h3_index, child_res)` - Get child cells at higher resolution
- `h3_distance(h3_index1, h3_index2)` - Grid distance between cells
- `h3_k_ring(h3_index, k)` - Get cells within k distance
- `h3_resolution(h3_index)` - Get resolution of H3 cell
- `h3_is_valid(h3_index)` - Validate H3 index

### Virtual Table
- `h3_spatial` - Virtual table for spatial queries and indexing
- Support for range queries, nearest neighbor searches
- Integration with existing spatial data

## H3 geospatial index intro

Why?
- Hexagons are better than squares for geo indexing: distances from one hexagon center to all its neighbors are the same
while they are different for squares.

How?
- H3 has 16 different grid sizes, called resolutions. They are numbered from 0 to 15.
- Resolution 0 covers the whole world with 122 massive hexagons.
- Resolution 15 is the most fine-grained, hexagons ~ 0.9 square meters.
- Each parent hexagon contains `7 smaller hexagons` of the next resolution.

Example
- Database with millions of points - locations of every coffee shops in a country.
- Each location stored with its precise (resolution 10) H3 index.
- Task: create a heatmap to show the density of coffee shops across a large city.
  
```sql
-- Get the resolution-7 hexagon index to display data on a zoomed out hexagon.
-- h3_parent(h3_resolution_10_index, 7) is very fast, not geometric calculation is needed.
SELECT
   h3_parent(h3_resolution_10_index, 7) AS h3_resolution_7_index,
   COUNT(*) AS number_of_shops
FROM
   coffee_shops
GROUP BY
   h3_resolution_7_index;
```

What is the data to represent H3 index?

## Usage Examples

### Basic Coordinate Conversion
```sql
-- Convert latitude/longitude to H3 cell at resolution 9
SELECT h3_from_latlng(37.7749, -122.4194, 9) as h3_cell;

-- Get center coordinates of H3 cell
SELECT h3_to_latlng('8928308280fffff') as center;
```

### Spatial Queries
```sql
-- Find all points within 2 km of a location
SELECT * FROM locations 
WHERE h3_cell IN (
    SELECT h3_k_ring(h3_from_latlng(37.7749, -122.4194, 9), 2)
);

-- Get neighboring cells
SELECT h3_neighbors(h3_from_latlng(37.7749, -122.4194, 9));
```

### Hierarchical Operations
```sql
-- Get parent cell at lower resolution
SELECT h3_parent(h3_from_latlng(37.7749, -122.4194, 9), 7);

-- Get child cells at higher resolution
SELECT h3_children(h3_from_latlng(37.7749, -122.4194, 7), 9);
```

## References

- [H3 Geospatial Indexing System](https://h3geo.org/)
- [h3o Rust Library](https://github.com/HydroniumLabs/h3o)
- [Turso Extension API Documentation](../core/README.md)
- [H3 Technical Specification](https://h3geo.org/docs/core-library/restable/)
