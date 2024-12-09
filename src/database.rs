use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::passwords; // Import the schema module
use crate::schema::passwords::dsl as passwords_dsl; // Alias the DSL for operations

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
    SqliteConnection::establish("passwords.db")
        .expect("Error connecting to the database")
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
