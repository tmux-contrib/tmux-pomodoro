use rusqlite::{Connection, Result};
use xdg::BaseDirectories;

// DATABABSE_SCHEMA is a string that contains the SQL schema for the database.
// It is included at compile time.
const DATABASE_SCHEMA: &str = include_str!("schema.sql");

// Database is a struct that represents the connection to the SQLite database.
// It provides methods for migrating the database schema.
pub struct Database {
    conn: Connection,
}

impl Database {
    // New creates a new instance of the Database struct.
    // It opens a connection to the "pomodoro.db" SQLite.
    pub fn new() -> Self {
        Database {
            conn: Self::connect(),
        }
    }

    // Connect opens a connection to the "pomodoro.db" SQLite database and returns it.
    pub fn connect() -> Connection {
        let path = BaseDirectories::with_prefix("pomodoro")
            .place_state_file("pomodoro.db")
            .expect("Unable to create state file path");
        // Connect to the SQLite database at the specified path.
        Connection::open(path).expect("Failed to connect to database")
    }

    // Migrate applies the database schema to the database. It executes the SQL commands defined in
    // the DATABABSE_SCHEMA constant, which creates the necessary tables and structures for the application
    // to function properly. If the migration is successful, it returns Ok(()); otherwise, it
    // returns an error.
    pub fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(DATABASE_SCHEMA)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_is_migrated() {
        let database = Database::new();
        let result = database.migrate();
        assert!(
            result.is_ok(),
            "Database migration failed: {:?}",
            result.err()
        );
    }
}
