use std::env;

/// Get the database path from environment variable or use default
pub fn get_db_path() -> String {
    env::var("LIBRO_DB_PATH").unwrap_or_else(|_| "libro.db".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_db_path() {
        // Save current value if it exists
        let original = env::var("LIBRO_DB_PATH").ok();

        env::remove_var("LIBRO_DB_PATH");
        assert_eq!(get_db_path(), "libro.db");

        // Restore original value if it existed
        if let Some(value) = original {
            env::set_var("LIBRO_DB_PATH", value);
        }
    }

    #[test]
    fn test_custom_db_path() {
        // Save current value if it exists
        let original = env::var("LIBRO_DB_PATH").ok();

        env::set_var("LIBRO_DB_PATH", "/tmp/test.db");
        assert_eq!(get_db_path(), "/tmp/test.db");

        // Restore original value or remove if it didn't exist
        match original {
            Some(value) => env::set_var("LIBRO_DB_PATH", value),
            None => env::remove_var("LIBRO_DB_PATH"),
        }
    }
}
