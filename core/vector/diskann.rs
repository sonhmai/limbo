use crate::Connection;
use crate::result::LimboResult;
use crate::vector::vector_types::Vector;
use crate::vector::VectorOutputRows;

struct DiskAnnIndex {
    connection: Connection,
    index_name: String,
}

impl DiskAnnIndex {
    pub fn create_index(connection: Connection, index_name: String) -> Self {
        todo!()
    }

    pub fn insert(&mut self) {
        todo!()
    }

    pub fn delete(&mut self) {
        todo!()
    }

    /// Search `k` nearest neighbours for `vector` in this index.
    pub fn search(&self, vector: Vector, k: usize) -> Option<VectorOutputRows> {
        todo!()
    }
}