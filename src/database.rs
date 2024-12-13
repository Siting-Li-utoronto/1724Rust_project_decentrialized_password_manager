use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::passwords; // Import the schema module
use crate::schema::passwords::dsl as passwords_dsl; // Alias the DSL for operations
use std::env;

#[derive(Queryable)]
pub struct Password {
    pub id: i32,
    pub title: String,
    pub encrypted_password: String,
}

#[derive(Insertable)]
#[table_name = "passwords"] // Link this struct to the passwords table
pub struct NewPassword<'a> {
    pub title: &'a str,
    pub encrypted_password: &'a str,
}

/// Establishes a connection to the SQLite database.
pub fn establish_connection() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "passwords.db".to_string());
    SqliteConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

/// Creates the `passwords` table if it does not already exist.
pub fn create_table_if_not_exists() {
    let connection = establish_connection();

    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            encrypted_password TEXT NOT NULL
        );",
    )
    .execute(&connection)
    .expect("Failed to create table");
}

/// Cleans the database by dropping and recreating the `passwords` table.
pub fn clean_database() {
    let connection = establish_connection();

    // Drop the table if it exists
    diesel::sql_query("DROP TABLE IF EXISTS passwords;")
        .execute(&connection)
        .expect("Failed to drop table");

    // Recreate the table
    create_table_if_not_exists();
}

/// Adds a new password to the database.
pub fn add_password(title: &str, encrypted_password: &str) -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    // Check if the password title already exists
    let existing_password = passwords_dsl::passwords
        .filter(passwords_dsl::title.eq(title))
        .first::<Password>(&connection)
        .optional()?;

    if existing_password.is_some() {
        return Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new(String::from("A password with the given title already exists.")),
        ));
    }

    // Create a new password struct for insertion
    let new_password = NewPassword {
        title,
        encrypted_password,
    };

    // Insert the new password
    diesel::insert_into(passwords::table)
        .values(&new_password)
        .execute(&connection)
        .map(|_| ())
}

/// Retrieves a password from the database by its title.
pub fn get_password(title: &str) -> Option<String> {
    let connection = establish_connection();

    passwords_dsl::passwords // Use the aliased DSL for querying
        .filter(passwords_dsl::title.eq(title))
        .select(passwords_dsl::encrypted_password)
        .first::<String>(&connection)
        .optional()
        .expect("Failed to retrieve password")
}

/// Updates the encrypted password for a given title.
pub fn update_password(title: &str, new_encrypted_password: &str) -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    // Find the password entry by title and update the encrypted password
    let updated_rows = diesel::update(passwords_dsl::passwords.filter(passwords_dsl::title.eq(title)))
        .set(passwords_dsl::encrypted_password.eq(new_encrypted_password))
        .execute(&connection)?;

    // If no rows were updated, return an error
    if updated_rows == 0 {
        return Err(diesel::result::Error::NotFound);
    }

    Ok(())
}

/// Deletes a password entry by its title.
/// Returns `Ok(())` if successful, or an error if the title does not exist.
pub fn delete_password(title: &str) -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    // Delete the password where title matches
    let deleted_rows = diesel::delete(passwords_dsl::passwords.filter(passwords_dsl::title.eq(title)))
        .execute(&connection)?;

    // If no rows were deleted, return an error
    if deleted_rows == 0 {
        return Err(diesel::result::Error::NotFound);
    }

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    /// Setup: Configure the test environment with an isolated test database.
    fn setup_test_database() {
        // Set a separate database for testing
        env::set_var("DATABASE_URL", "test_passwords.db");

        // Clean up any existing test database
        let _ = fs::remove_file("test_passwords.db");

        // Recreate the table for testing
        create_table_if_not_exists();
    }

    #[test]
    fn test_create_table() {
        // Setup: Initialize the test database
        setup_test_database();

        // Act: Create the table (already handled in setup)
        create_table_if_not_exists();

        // Assert: Table creation should succeed (no panic expected)
        assert!(true, "Table creation should not fail.");
    }

    #[test]
    fn test_add_and_retrieve_password() {
        // Setup: Initialize the test database
        setup_test_database();

        // Arrange
        let title = "test_entry";
        let encrypted_password = "test_encrypted_password";

        // Act: Add a password
        let result = add_password(title, encrypted_password);
        assert!(
            result.is_ok(),
            "Failed to add password: {:?}",
            result.err()
        );

        // Act: Retrieve the password
        let retrieved_password = get_password(title);

        // Assert
        assert_eq!(
            retrieved_password,
            Some(encrypted_password.to_string()),
            "Retrieved password does not match the expected value."
        );
    }

    #[test]
    fn test_retrieve_nonexistent_password() {
        // Setup: Initialize the test database
        setup_test_database();

        // Arrange
        let title = "nonexistent_title";

        // Act: Attempt to retrieve a nonexistent password
        let result = get_password(title);

        // Assert: The result should be None
        assert_eq!(
            result,
            None,
            "Expected None for nonexistent password, but got {:?}",
            result
        );
    }

    #[test]
    fn test_duplicate_password() {
        // Setup: Initialize the test database
        setup_test_database();

        // Arrange
        let title = "duplicate_entry";
        let encrypted_password = "test_password";

        // Act: Add the password for the first time
        let result1 = add_password(title, encrypted_password);
        assert!(
            result1.is_ok(),
            "Failed to add first password: {:?}",
            result1.err()
        );

        // Act: Attempt to add the same password again (duplicate)
        let result2 = add_password(title, encrypted_password);

        // Assert: The second addition should fail
        assert!(
            result2.is_err(),
            "Adding a duplicate password should fail, but succeeded."
        );
    }

    #[test]
    fn test_clean_database() {
        // Setup: Initialize the test database
        setup_test_database();

        // Arrange: Add a test entry
        let title = "clean_test_entry";
        let encrypted_password = "test_password";
        let result = add_password(title, encrypted_password);
        assert!(
            result.is_ok(),
            "Failed to add password before cleaning: {:?}",
            result.err()
        );

        // Act: Clean the database
        clean_database();

        // Assert: The entry should no longer exist
        let retrieved_password = get_password(title);
        assert_eq!(
            retrieved_password,
            None,
            "Expected database to be empty after cleaning, but found data."
        );
    }
}

