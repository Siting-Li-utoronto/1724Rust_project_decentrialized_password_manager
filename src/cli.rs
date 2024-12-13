use clap::{App, Arg, SubCommand};
use crate::database::{add_password, get_password, clean_database};
use crate::encryption::{encrypt_password, decrypt_password, generate_key};

// main function for processing and handling CLI commands.
pub fn run_cli() {
    let matches = App::new("Decentralized Password Manager")
        .version("0.1")
        .author("Siting Li & Zifan Meng")
        .about("Secure decentralized password manager")
        .subcommand(
            // Takes the title and password, checks if they are valid, encrypts the password, and attempts to add it to the database.
            SubCommand::with_name("add")
                .about("Adds a new password: cargo run -- add --title/t <example_title> --password/p <example_password>")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short('t')
                        .takes_value(true)
                        .required(true)
                        .help("The title of the password"),
                )
                .arg(
                    Arg::with_name("password")
                        .long("password")
                        .short('p')
                        .takes_value(true)
                        .required(true)
                        .help("The password to store"),
                ),
        )
        // Retrieves the password for the given title, decrypts it using the provided nonce, and displays the decrypted password.
        .subcommand(
            SubCommand::with_name("get")
                .about("Retrieves a password: cargo run -- get --title/t <example_title> --nonce/n <your_nonce>")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short('t')
                        .takes_value(true)
                        .help("The title of the password to retrieve"), // Make title optional here
                )
                .arg(
                    Arg::with_name("nonce")
                        .long("nonce")
                        .short('n')
                        .takes_value(true)
                        .help("The nonce required for decryption"),
                ),
        )
        // Cleans the database by dropping and recreating the table.
        .subcommand(
            SubCommand::with_name("clean")
                .about("Cleans the database by removing all passwords: cargo run -- clean")
        )
        // Modify the password
        .subcommand(
            SubCommand::with_name("modify")
                .about("Modifies an existing password: cargo run -- modify --title/t <example_title> --new_password/p <new_password> --nonce/n <nonce>")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short('t')
                        .takes_value(true)
                        .required(true)
                        .help("The title of the password to modify"),
                )
                .arg(
                    Arg::with_name("new_password")
                        .long("new_password")
                        .short('p')
                        .takes_value(true)
                        .required(true)
                        .help("The new password to store"),
                )
                .arg(
                    Arg::with_name("nonce")
                        .long("nonce")
                        .short('n')
                        .takes_value(true)
                        .required(true)
                        .help("The nonce required for decrypting the old password"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a password: cargo run -- delete --title/t <example_title> --nonce/n <nonce>")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short('t')
                        .takes_value(true)
                        .required(true)
                        .help("The title of the password to delete"),
                )
                .arg(
                    Arg::with_name("nonce")
                        .long("nonce")
                        .short('n')
                        .takes_value(true)
                        .required(true)
                        .help("The nonce required for verifying the password"),
                ),
        )

        .get_matches();

    let key = generate_key();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.value_of("title").unwrap();
        let password = matches.value_of("password").unwrap();

        if title.is_empty() {
            println!("Failed to add password: Title cannot be empty.");
            return;
        }

        // Check if the password is empty
        if password.is_empty() {
            println!("Failed to add password: Password cannot be empty.");
            return;
        }

        // Encrypt password first
        let (encrypted_password, nonce) = encrypt_password(password, &key);

        // Attempt to add the password to the database
        match add_password(title, &encrypted_password) {
            Ok(_) => {
                // Only display nonce if password is successfully added
                println!(
                    "Password added for '{}'. Store this nonce securely: {}",
                    title, nonce
                );
            }
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => println!("Failed to add password: A password with the title '{}' already exists.", title),
            Err(e) => panic!("Failed to add password: {:?}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        // Check if title is provided; if not, print error and exit
        if let Some(title) = matches.value_of("title") {
            let nonce = matches.value_of("nonce");

            if nonce.is_none() {
                println!("Error: Missing required argument --nonce for decryption");
                return;
            }

            if let Some(encrypted_password) = get_password(title) {
                match decrypt_password(&encrypted_password, nonce.unwrap(), &key) {
                    Ok(decrypted_password) => {
                        println!("Decrypted password for '{}': {}", title, decrypted_password);
                    }
                    Err(err) => {
                        println!("Failed to decrypt password for '{}': {}", title, err);
                    }
                }
            } else {
                println!("No password found for '{}'.", title);
            }
        } else {
            println!("Error: Missing required argument --title for retrieving the password");
        }
    } else if matches.subcommand_matches("clean").is_some() {
        clean_database(); // This cleans the database
        println!("Database cleaned and reset."); // Print only once
    } else if let Some(matches) = matches.subcommand_matches("modify") {
        let title = matches.value_of("title").unwrap();
        let new_password = matches.value_of("new_password").unwrap();
        let nonce = matches.value_of("nonce").unwrap();

        let key = generate_key();

        // Step 1: Retrieve and decrypt the old password
        if let Some(encrypted_password) = get_password(title) {
            match decrypt_password(&encrypted_password, nonce, &key) {
                Ok(_) => {
                    // Step 2: Encrypt the new password
                    let (new_encrypted_password, new_nonce) = encrypt_password(new_password, &key);

                    // Step 3: Update the password in the database
                    match crate::database::update_password(title, &new_encrypted_password) {
                        Ok(_) => {
                            println!(
                                "Password updated for '{}'. Store this new nonce securely: {}",
                                title, new_nonce
                            );
                        }
                        Err(diesel::result::Error::NotFound) => {
                            println!("No password found for '{}'. Update failed.", title);
                        }
                        Err(e) => {
                            println!("Failed to update password: {:?}", e);
                        }
                    }
                }
                Err(err) => println!("Failed to decrypt old password: {}", err),
            }
        } else {
            println!("No password found for '{}' to update.", title);
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let title = matches.value_of("title").unwrap();
        let nonce = matches.value_of("nonce").unwrap();

        let key = generate_key();

        // Step 1: Retrieve the encrypted password for the given title
        if let Some(encrypted_password) = get_password(title) {
            // Step 2: Decrypt the password using the provided nonce and key
            match decrypt_password(&encrypted_password, nonce, &key) {
                Ok(_) => {
                    // Step 3: Delete the password entry
                    match crate::database::delete_password(title) {
                        Ok(_) => println!("Password with title '{}' has been successfully deleted.", title),
                        Err(diesel::result::Error::NotFound) => {
                            println!("No password found for '{}' to delete.", title);
                        }
                        Err(e) => {
                            println!("Failed to delete password: {:?}", e);
                        }
                    }
                }
                Err(err) => println!("Failed to verify password: {}", err),
            }
        } else {
            println!("No password found for '{}' to delete.", title);
        }
    }


}
