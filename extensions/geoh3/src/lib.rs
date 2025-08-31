use turso_ext::register_extension;

mod functions;
mod types;
mod utils;

pub use functions::*;
pub use types::*;

// Register all H3 geospatial functions with Turso
register_extension! {
    scalars: {
        h3_from_latlng,
        h3_to_latlng,
        h3_to_boundary,
        h3_neighbors,
        h3_parent,
        h3_children,
        h3_distance,
        h3_k_ring,
        h3_resolution,
        h3_is_valid
    },
    aggregates: {},
    vtabs: {},
    vfs: {},
}
