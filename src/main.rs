#[macro_use]
extern crate diesel;

mod database;
mod encryption;
mod cli;
mod schema;

use cli::run_cli;

fn main() {
    // create the necessary database table if it doesn't already exist.
    database::create_table_if_not_exists();

    // start the command-line interface (CLI) for user interaction.
    run_cli();
}
