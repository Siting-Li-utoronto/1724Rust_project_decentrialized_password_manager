use std::process::Command;

#[test]
fn test_cli_duplicate_password_handling() {
    // Step 1: Add a new password
    let output1 = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "add",
            "--title",
            "test_duplicate_cli",
            "--password",
            "test_password_123",
        ])
        .output()
        .expect("Failed to execute first add command");

    let stdout1 = String::from_utf8(output1.stdout).expect("Invalid UTF-8 in first add output");

    // Assert: First addition should succeed
    assert!(
        stdout1.contains("Password added for 'test_duplicate_cli'"),
        "First addition failed unexpectedly"
    );

    // Step 2: Attempt to add the same password with the same title again
    let output2 = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "add",
            "--title",
            "test_duplicate_cli",
            "--password",
            "test_password_123",
        ])
        .output()
        .expect("Failed to execute second add command");

    let stdout2 = String::from_utf8(output2.stdout).expect("Invalid UTF-8 in second add output");

    // Assert: Duplicate addition should fail and output an appropriate message
    assert!(
        stdout2.contains("Failed to add password: A password with the title 'test_duplicate_cli' already exists."),
        "CLI did not handle duplicate title correctly"
    );

    println!("Duplicate password CLI test passed successfully!");
}

#[test]
fn test_cli_help() {
    // Act: Run the help command
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in output");

    // Assert
    assert!(stdout.contains("Decentralized Password Manager"));
    assert!(stdout.contains("--title"));
    assert!(stdout.contains("--password"));
}

#[test]
fn test_cli_add_password() {
    // Act: Add a new password
    let output = Command::new("cargo")
        .args(&[
            "run", "--", "add", "--title", "test_title", "--password", "test_password",
        ])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in output");

    // Assert
    assert!(stdout.contains("Password added for 'test_title'"));
}

#[test]
fn test_cli_get_password() {
    // Step 1: Add a password and capture the output
    let add_output = Command::new("cargo")
        .args(&[
            "run", "--", "add", "--title", "test_get", "--password", "test_password",
        ])
        .output()
        .expect("Failed to add password");

    let add_stdout = String::from_utf8(add_output.stdout).expect("Invalid UTF-8 in add output");

    // Extract the nonce from the "add" command output
    let nonce_line = add_stdout
        .lines()
        .find(|line| line.contains("Store this nonce securely:"))
        .expect("Nonce not found in add command output");

    let nonce = nonce_line
        .split("Store this nonce securely: ")
        .nth(1)
        .expect("Failed to extract nonce")
        .trim();

    println!("Extracted Nonce: {}", nonce); // Debugging output

    // Step 2: Retrieve the password using the captured nonce
    let get_output = Command::new("cargo")
        .args(&[
            "run", "--", "get", "--title", "test_get", "--nonce", nonce,
        ])
        .output()
        .expect("Failed to retrieve password");

    let get_stdout = String::from_utf8(get_output.stdout).expect("Invalid UTF-8 in get output");

    // Assert: Verify the password was decrypted successfully
    assert!(
        get_stdout.contains("Decrypted password for 'test_get': test_password"),
        "Expected decrypted password not found in output. Actual output: {}",
        get_stdout
    );
}

#[test]
fn test_cli_clean_database() {
    // Act: Run the clean command
    let output = Command::new("cargo")
        .args(&["run", "--", "clean"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in output");

    // Assert
    assert!(stdout.contains("Database cleaned and reset"));
}

#[test]
fn test_cli_modify_password() {
    // Step 1: Add a new password
    let add_output = Command::new("cargo")
        .args(&[
            "run", "--", "add", "--title", "modify_test", "--password", "old_password",
        ])
        .output()
        .expect("Failed to add password");

    let add_stdout = String::from_utf8(add_output.stdout).expect("Invalid UTF-8 in add output");

    // Extract the nonce from the "add" command output
    let nonce_line = add_stdout
        .lines()
        .find(|line| line.contains("Store this nonce securely:"))
        .expect("Nonce not found in add command output");

    let old_nonce = nonce_line
        .split("Store this nonce securely: ")
        .nth(1)
        .expect("Failed to extract nonce")
        .trim();

    // Step 2: Modify the password and capture the new nonce
    let modify_output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "modify",
            "--title",
            "modify_test",
            "--new_password",
            "new_password",
            "--nonce",
            old_nonce,
        ])
        .output()
        .expect("Failed to modify password");

    let modify_stdout = String::from_utf8(modify_output.stdout).expect("Invalid UTF-8 in modify output");

    let new_nonce_line = modify_stdout
        .lines()
        .find(|line| line.contains("Store this new nonce securely:"))
        .expect("New nonce not found in modify command output");

    let new_nonce = new_nonce_line
        .split("Store this new nonce securely: ")
        .nth(1)
        .expect("Failed to extract new nonce")
        .trim();

    // Step 3: Retrieve the updated password using the new nonce
    let get_output = Command::new("cargo")
        .args(&[
            "run", "--", "get", "--title", "modify_test", "--nonce", new_nonce,
        ])
        .output()
        .expect("Failed to retrieve password");

    let get_stdout = String::from_utf8(get_output.stdout).expect("Invalid UTF-8 in get output");

    // Assert: Verify the updated password
    assert!(
        get_stdout.contains("Decrypted password for 'modify_test': new_password"),
        "Updated password retrieval failed. Output: {}",
        get_stdout
    );
}

#[test]
fn test_cli_delete_password() {
    // Step 1: Add a new password
    let add_output = Command::new("cargo")
        .args(&[
            "run", "--", "add", "--title", "delete_test", "--password", "delete_password",
        ])
        .output()
        .expect("Failed to add password");

    let add_stdout = String::from_utf8(add_output.stdout).expect("Invalid UTF-8 in add output");

    // Extract the nonce from the "add" command output
    let nonce_line = add_stdout
        .lines()
        .find(|line| line.contains("Store this nonce securely:"))
        .expect("Nonce not found in add command output");

    let nonce = nonce_line
        .split("Store this nonce securely: ")
        .nth(1)
        .expect("Failed to extract nonce")
        .trim();

    // Step 2: Delete the password
    let delete_output = Command::new("cargo")
        .args(&[
            "run", "--", "delete", "--title", "delete_test", "--nonce", nonce,
        ])
        .output()
        .expect("Failed to delete password");

    let delete_stdout = String::from_utf8(delete_output.stdout).expect("Invalid UTF-8 in delete output");

    assert!(
        delete_stdout.contains("Password with title 'delete_test' has been successfully deleted."),
        "Failed to delete password"
    );

    // Step 3: Verify the password no longer exists
    let get_output = Command::new("cargo")
        .args(&[
            "run", "--", "get", "--title", "delete_test", "--nonce", nonce,
        ])
        .output()
        .expect("Failed to retrieve password");

    let get_stdout = String::from_utf8(get_output.stdout).expect("Invalid UTF-8 in get output");

    assert!(
        get_stdout.contains("No password found for 'delete_test'"),
        "Password was not deleted successfully"
    );
}
