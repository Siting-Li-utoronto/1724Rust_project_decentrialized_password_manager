// Defines the structure of the passwords table for Diesel ORM.
table! {
    passwords (id) {
        id -> Integer,
        title -> Text,
        encrypted_password -> Text,
    }
}
