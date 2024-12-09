# **1724 RUST Final Project - Decentralized Password Manager**

A decentralized password manager built in Rust using SQLite and encryption.

## **Team Members**

- **Siting Li** (1005046335) - lisitin1 - siting.li@mail.utoronto.ca
- **Zifan Meng** (1005364730) - mengzifa - zifan.meng@mail.utoronto.ca

---

## **Motivation**

The motivation behind this project stems from the increasing need for secure and privacy-focused solutions in the digital age. With the proliferation of online accounts and services, people are increasingly relying on password managers to securely store and manage their passwords. However, existing centralized password management solutions pose significant privacy and security risks. Many of these systems store user passwords in centralized databases, making them prime targets for cyberattacks. Additionally, users often have to trust these systems to keep their passwords secure, leaving them vulnerable in case of a data breach or misuse.

Our motivation for this project is to build a password manager that addresses these risks by adopting a **decentralized** approach. By using peer-to-peer networking and strong cryptographic methods, we aim to create a password manager where sensitive data is not stored in a single centralized database. This ensures that even if one device is compromised, the user's passwords remain secure, as the data is never stored in plaintext on any server. Additionally, users will have complete control over their data, empowering them to manage their passwords securely without relying on third-party service providers.

By building this decentralized password manager in Rust, we are leveraging the power of memory safety, performance, and modern encryption techniques to offer a more secure and private alternative to traditional password management tools. This project fulfills a gap in the ecosystem, providing a solution that emphasizes security, privacy, and decentralization in managing sensitive information.

---

## **Objectives**

The primary objective of this project is to develop a decentralized and secure password manager that guarantees user privacy through encryption and a zero-knowledge architecture. The system will provide a simple and secure interface for storing and retrieving passwords while ensuring data security, even in a decentralized environment.

### **Key Objectives:**

1. **End-to-End Encryption**: Implement encryption mechanisms to ensure that passwords are encrypted locally before being stored, with no unencrypted data transmitted over the network. We will use the Sodiumoxide library for cryptographic security to protect passwords both in transit and at rest.

2. **Decentralization**: Create a peer-to-peer (P2P) network that allows devices to communicate directly, avoiding the use of centralized servers. This reduces the risks of data breaches since passwords are not stored in a single location.

3. **Zero-Knowledge Architecture**: Ensure that only the user can access their plaintext passwords by using a zero-knowledge model. Even the developers and administrators will have no access to the users' actual data, ensuring complete privacy.

4. **User-Friendly Command-Line Interface (CLI)**: Design an intuitive CLI that allows users to add, retrieve, and manage their passwords easily. The tool should be easy to use for both casual and power users, ensuring broad accessibility.

5. **Security and Integrity**: Build the system to handle password data securely, including robust encryption methods, secure hashing for master passwords, and thorough testing to identify potential vulnerabilities.

6. **Database Integration**: Implement SQLite for local storage, using it to securely store encrypted passwords on the user's device. SQLite will provide a lightweight and reliable database solution for local data management.

---

## **Features**

The decentralized password manager provides a variety of features designed to ensure secure password management while offering full control to the user. Below are the key features of the final project deliverable:

### 1. **End-to-End Encryption**
   - All passwords are encrypted locally on the user¡¯s device using strong cryptographic algorithms before being stored or transmitted over the network.
   - The Sodiumoxide library, a Rust wrapper around NaCl (a high-level cryptography library), is used to perform encryption and decryption operations.
   - Passwords are encrypted with a unique nonce (number used once), ensuring that each encryption is distinct and preventing certain types of cryptographic attacks.

### 2. **Zero-Knowledge Architecture**
   - The password manager uses a **zero-knowledge** model, meaning that even the system¡¯s creators or administrators do not have access to the users' plaintext passwords.
   - Only the user can decrypt their passwords using their unique cryptographic keys, ensuring that sensitive data remains private, even if the system is compromised.

### 3. **Peer-to-Peer (P2P) Synchronization**
   - Devices can synchronize passwords over a decentralized peer-to-peer network, eliminating the need for central servers.
   - Passwords are never stored centrally, reducing the risk of data breaches. Each device in the network can securely share and synchronize encrypted passwords.

### 4. **Command-Line Interface (CLI)**
   - A user-friendly CLI allows users to perform key password management actions easily:
     - **Add passwords**: Store new passwords securely with encryption.
     - **Retrieve passwords**: Decrypt and view stored passwords when needed.
     - **Clean the database**: Remove all stored data from the local database when needed.
   - Commands are simple and easy to use, with clear options for interacting with the system.

### 5. **Local Database Storage (SQLite)**
   - SQLite is used for local storage, where all passwords are stored encrypted, ensuring they are never exposed in plaintext.
   - SQLite provides an efficient and lightweight database solution for storing password data securely.

### 6. **Secure Password Retrieval**
   - Passwords are retrieved securely from the local database and decrypted on the fly when the user needs access.
   - Decryption is done using the correct **nonce** and encryption key, ensuring that only the rightful owner of the password can decrypt it.

### 7. **Validation of Input**
   - The password manager ensures that input data (such as titles and passwords) is valid before adding them to the database.
   - Empty or invalid inputs are rejected, and the user is prompted with a helpful error message to correct the issue.
   
### 8. **Secure Password Duplication Check**
   - When attempting to add a password with a title that already exists in the database, the system prevents duplication by checking for existing entries.
   - If a password with the same title already exists, the user is informed, preventing overwriting of existing passwords.

### 9. **Automatic Key Generation**
   - A random encryption key is generated for each user to ensure strong cryptographic protection of their data.
   - The key is used for encrypting and decrypting passwords and is never stored with the encrypted data, enhancing security.

### 10. **Database Cleaning Feature**
   - The ability to clean or reset the entire password database allows users to delete all stored passwords from the local storage if desired.
   - This ensures that the application can be cleared of all sensitive data when no longer needed.

These features together create a powerful, secure, and user-friendly decentralized password manager that ensures privacy, security, and control over sensitive information.

---

## **User's (or Developer's) Guide**

This guide will help users (or developers) understand how to interact with the decentralized password manager, whether using it as an end-user or as a developer working with the source code.

### 1. **Installation**

#### **Prerequisites**

Ensure that the following are installed:
   - **Rust**: Install from [rust-lang.org](https://rust-lang.org).
   - **SQLite**: Install via your package manager.
   
#### **Clone the Repository**

To begin using or contributing to the project, clone the repository and install the necessary dependencies:

```bash
git clone https://github.com/Siting-Li-utoronto/1724Rust_project_decentrialized_password_manager.git
cd decentralized-password-manager
```

#### **Build the Project**

```bash
cargo build
```

#### **Run the Application**

To see what command can be used:

```bash
cargo run -- --help
```

---

### 2. **Command-Line Interface (CLI) Features**

The CLI tool allows you to add, retrieve, and clean passwords securely. All commands follow this format:

```bash
cargo run -- <command> [options]
```

The following are examples for running this decentrialized password manager.

#### 2.1 **Adding a Password**

To securely add a password:

```bash
cargo run -- add --title "example_title" --password "example_password" 
```

##### **Options**

- `--title` or `-t`: Title for the password.
- `--password` or `-p`: The password you want to store.

##### **Output:**
If successful, you will see:

```text
Password added for 'example_title'. Store this nonce securely: <nonce>  
```

**Note**: The **nonce** is critical for decryption. Save it securely; without it, you cannot retrieve the password.

---

#### 2.2 **Retrieving a Password**

To retrieve a password:

```bash  
cargo run -- get --title "example_title" --nonce "your_nonce"  
```

##### **Options:**
- `--title` or `-t`: Title of the password to retrieve.
- `--nonce` or `-n`: Nonce required for decryption (provided when you added the password).

##### **Output:**
If successful:

```text  
Decrypted password for 'example_title': your_password_here  
```

##### **Example:**

```bash  
cargo run -- get --title "email_account" --nonce "Base64NonceHere"  
```

---

#### 2.3 **Cleaning the Database**

To reset the password database by deleting all stored entries:

```bash  
cargo run -- clean  
```

##### **Output:**

```text  
Database cleaned and reset.  
```

**Note**: This action is **irreversible**. All stored passwords will be deleted.

---

### 3. **Understanding Encryption and Keys**

#### **How it Works**

- Passwords are encrypted **locally** before storage using the **Sodiumoxide** library.
- Each encryption generates a unique **nonce** and uses a **32-byte key**.
- Decryption requires both the encrypted password and the nonce.

---

#### **Key Generation**

- A key is generated automatically at runtime using a static placeholder.

#### **Security Notes:**
1. **Never lose your nonce**: Without the nonce, decryption is impossible.
2. **Store encryption keys securely**: Do not expose your key in production environments.

---

### 4. **Developer Notes**

#### **Code Structure**

- **`main.rs`**: Entry point for the application.
- **`cli.rs`**: Handles command-line commands using the Clap library.
- **`database.rs`**: Manages SQLite interactions (create table, add, retrieve, clean).
- **`encryption.rs`**: Handles password encryption and decryption.
- **`schema.rs`**: Defines the database schema for Diesel ORM.

---

#### **Testing**

To test the application, use:

```bash  
cargo test -- --test-threads=1
```

---

#### **Dependencies**

- **Clap**: For parsing CLI commands.
- **Diesel**: For database interactions with SQLite.
- **Sodiumoxide**: For encryption and cryptographic functions.
- **Base64**: For encoding encrypted data and nonces.

---

### 5. **Error Handling**

#### **Common Errors and Fixes**

| **Error**                                       | **Cause**                               | **Solution**                               |
|-------------------------------------------------|----------------------------------------|-------------------------------------------|
| `Failed to add password: Title cannot be empty.` | Title was not provided.                | Use `--title` with a valid string.         |
| `Failed to decode ciphertext`                   | Invalid nonce or encrypted password.   | Ensure correct nonce and encrypted data.   |
| `Error: Missing required argument --nonce`      | Missing `--nonce` when using `get`.    | Always provide the correct nonce.          |
| `Failed to connect to database`                 | Database file issue.                   | Ensure SQLite is installed and accessible. |
| `No password found for 'title'`                 | Title does not exist in the database.  | Double-check the title you entered.        |

---

## **Reproducibility Guide**

### 1. **Installation**

#### **Prerequisites**

Ensure that the following are installed:
   - **Rust**: Install from [rust-lang.org](https://rust-lang.org).
   
#### **Clone the Project Repository**

To begin using or contributing to the project, clone the repository and install the necessary dependencies:

```bash
git clone https://github.com/Siting-Li-utoronto/1724Rust_project_decentrialized_password_manager.git
cd decentralized-password-manager
```

#### **Build the Project**

```bash
cargo build
```

#### **Run the Project**

You can now run the application using Cargo. Start by checking the help menu to verify the setup:

```bash
cargo run -- --help
```

You should see a list of available commands and options.

---

### 2. **Using the CLI Features**

#### 2.1 **Adding a Password**

To securely add a password, run:

```bash
cargo run -- add --title "example_title" --password "example_password" 
```

##### **Example:**

```bash
cargo run -- add --title "email_account" --password "myp@ssword123"
```

##### **Output:**

```text
Password added for 'email_account'. Store this nonce securely: <nonce>  
```

**Note**: The **nonce** is critical for decryption. Save it securely; without it, you cannot retrieve the password.

---

#### 2.2 **Retrieving a Password**

To retrieve a stored password, you must provide the title and nonce:

```bash  
cargo run -- get --title "example_title" --nonce "your_nonce_here"
```

##### **Example:**

```bash
cargo run -- get --title "email_account" --nonce "your_nonce_here"
```

##### **Output:**

```text  
Decrypted password for 'email_account': myp@ssword123 
```

---

#### 2.3 **Cleaning the Database**

To reset the database (delete all stored passwords), use:

```bash  
cargo run -- clean  
```

##### **Output:**

```text  
Database cleaned and reset.  
```

**Note**: This action is **irreversible**. All stored passwords will be deleted.

---

#### 2.4 **Testing the Project**

```bash  
cargo test -- --test-threads=1 
```

##### **Output:**

Total 12 testcases:

Unit tests:

```text
database.rs:
- Add and retrieve a password
- Clean the database
- Create the table
- Duplicate password
- Retrieve nonexistent password
encryption.rs:
- Decrypt with invalid nonce
- encrypt and decrypt password
```

Integration tests:

```text
- add password
- clean database
- duplicate password handling
- get password using title and nonce
- help command
```

The project should pass all tests.

---

## **Contributions by Each Team Member**

The following outlines the individual contributions made by each team member to the project:

### **1. Siting Li (1005046335)**

- **Project Design**:
  - Conceptualized the overall structure of the Decentralized Password Manager.

- **Implementation**:
  - Developed the **database module** (`database.rs`), which handles the SQLite operations:
    - Creating tables (`create_table_if_not_exists`).
    - Adding passwords (`add_password`).
    - Retrieving passwords (`get_password`).
    - Cleaning the database (`clean_database`).
  - Developed the **encryption module** (`encryption.rs`), including:
    - Password encryption using the **Sodiumoxide** library (`encrypt_password`).
    - Password decryption (`decrypt_password`).
    - Key generation (`generate_key`).
  - Worked on the **CLI module** (`cli.rs`) to parse and execute user commands using the `clap` library.
  - Integrated encryption into the database module to ensure secure storage of passwords.

- **Testing**:
  - Wrote the testcases for both unit tests and integration tests.

- **Documentation**:
  - Completed the project report.

---

### **2. Zifan Meng (1005364730)**

- **Project Design**:
- Designed the initial architecture for encryption, database integration, and CLI functionality.

- **CLI Enhancements**:
  - Refined the **CLI module** to handle edge cases:
    - Validation for empty titles or passwords.
    - Proper error handling for duplicate entries.
    - Improved error messages for missing arguments.

- **Testing**:
  - Ensured proper integration with the SQLite database.
  - Ensured that real nonces are captured and reused during CLI tests.

- **Debugging and Fixes**:
  - Resolved issues with the **"database is locked"** error by suggesting test isolation strategies.
  - Improved test reliability by using in-memory databases and isolating test data.

- **Documentation**:
  - Optimized final report.
  - Recorded demo video.

---

### **3. Team Collaboration**

- Both members actively collaborated on:
  - Debugging and integrating the modules.
  - Discussing architectural decisions and refining the project scope.
  - Ensuring that all code adhered to Rust best practices and was thoroughly tested.

---

## Lessons Learned and Concluding Remarks

### **Lessons Learned**

Throughout the development of the Decentralized Password Manager, the team gained valuable insights and overcame numerous challenges that contributed to our growth as developers and collaborators:

1. **Importance of Modular Design**:
   - We learned that breaking down the project into smaller, well-defined modules (e.g., `cli.rs`, `database.rs`, `encryption.rs`) made development, testing, and debugging significantly easier.
   - Clear separation of concerns allowed us to focus on individual functionalities while maintaining the integrity of the entire system.

2. **Effective Error Handling**:
   - Implementing robust error handling was crucial for a user-facing application.
   - Capturing errors at different layers (e.g., database errors, input validation errors) and displaying meaningful messages improved usability and resilience.

3. **Test-Driven Development**:
   - Writing comprehensive unit and integration tests ensured that each component behaved as expected, even as we added new features.
   - Managing test isolation by using separate databases or in-memory storage taught us the importance of reproducible and consistent test results.

4. **Understanding Rust's Ownership and Concurrency Model**:
   - As Rust emphasizes memory safety and ownership, we learned to manage data effectively, avoiding unnecessary clones and ensuring thread-safe operations.
   - Resolving issues like the "database is locked" error in SQLite highlighted the importance of managing resource access in concurrent scenarios.

5. **Working with Third-Party Libraries**:
   - Integrating libraries like `clap` for CLI parsing, `sodiumoxide` for encryption, and `diesel` for database operations expanded our understanding of external crates.
   - Learning to configure and use these libraries taught us how to leverage community tools to build secure and reliable software.

6. **Collaboration and Communication**:
   - Effective teamwork was essential to align our contributions, merge code seamlessly, and resolve conflicts.
   - Regular code reviews and discussions helped us identify potential improvements early, resulting in a cleaner and more maintainable codebase.

---

### **Concluding Remarks**

The Decentralized Password Manager project was a rewarding learning experience that combined our understanding of system design, security principles, and Rust programming. By building this tool:

- We delivered a lightweight, secure, and privacy-focused application that highlights the importance of decentralization and end-to-end encryption in managing sensitive data.
- We overcame challenges such as handling encryption keys, managing database state, and ensuring usability through a user-friendly CLI.

This project has strengthened our confidence in tackling real-world problems using Rust and has emphasized the importance of secure software development practices.

In conclusion, we believe that the Decentralized Password Manager demonstrates how strong cryptographic techniques and thoughtful software design can empower users to manage their passwords safely and independently. Moving forward, we aim to apply these lessons to build even more robust and secure solutions.

---


