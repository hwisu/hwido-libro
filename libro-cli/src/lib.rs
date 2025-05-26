pub mod lib {
    pub mod db;
    pub mod db_operations;
    pub mod errors;
    pub mod models;
}

// Re-export for easier access
pub use lib::*;
