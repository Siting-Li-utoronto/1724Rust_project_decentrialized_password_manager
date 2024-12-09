# 1724 RUST Final Project - Decentralized Password Manager

A decentralized password manager built in Rust using SQLite and encryption.

## Team Members

- **Siting Li** (1005046335) - lisitin1 - siting.li@mail.utoronto.ca
- **Zifan Meng** (12345678) - lalalala - abbccddee

## Motivation

The motivation behind this project stems from the increasing need for secure and privacy-focused solutions in the digital age. With the proliferation of online accounts and services, people are increasingly relying on password managers to securely store and manage their passwords. However, existing centralized password management solutions pose significant privacy and security risks. Many of these systems store user passwords in centralized databases, making them prime targets for cyberattacks. Additionally, users often have to trust these systems to keep their passwords secure, leaving them vulnerable in case of a data breach or misuse.

Our motivation for this project is to build a password manager that addresses these risks by adopting a **decentralized** approach. By using peer-to-peer networking and strong cryptographic methods, we aim to create a password manager where sensitive data is not stored in a single centralized database. This ensures that even if one device is compromised, the user's passwords remain secure, as the data is never stored in plaintext on any server. Additionally, users will have complete control over their data, empowering them to manage their passwords securely without relying on third-party service providers.

By building this decentralized password manager in Rust, we are leveraging the power of memory safety, performance, and modern encryption techniques to offer a more secure and private alternative to traditional password management tools. This project fulfills a gap in the ecosystem, providing a solution that emphasizes security, privacy, and decentralization in managing sensitive information.

## Objectives

The primary objective of this project is to develop a decentralized and secure password manager that guarantees user privacy through encryption and a zero-knowledge architecture. The system will provide a simple and secure interface for storing and retrieving passwords while ensuring data security, even in a decentralized environment.

### Key Objectives:

1. **End-to-End Encryption**: Implement encryption mechanisms to ensure that passwords are encrypted locally before being stored, with no unencrypted data transmitted over the network. We will use the Sodiumoxide library for cryptographic security to protect passwords both in transit and at rest.

2. **Decentralization**: Create a peer-to-peer (P2P) network that allows devices to communicate directly, avoiding the use of centralized servers. This reduces the risks of data breaches since passwords are not stored in a single location.

3. **Zero-Knowledge Architecture**: Ensure that only the user can access their plaintext passwords by using a zero-knowledge model. Even the developers and administrators will have no access to the users' actual data, ensuring complete privacy.

4. **User-Friendly Command-Line Interface (CLI)**: Design an intuitive CLI that allows users to add, retrieve, and manage their passwords easily. The tool should be easy to use for both casual and power users, ensuring broad accessibility.

5. **Security and Integrity**: Build the system to handle password data securely, including robust encryption methods, secure hashing for master passwords, and thorough testing to identify potential vulnerabilities.

6. **Database Integration**: Implement SQLite for local storage, using it to securely store encrypted passwords on the user's device. SQLite will provide a lightweight and reliable database solution for local data management.

## Features

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


## User's (or Developer¡¯s) Guide

### 1. **Installation**

To use the password manager, you must have `Rust` installed on your machine. You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/).

Once installed, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/Siting-Li-utoronto/1724Rust_project_decentralized_password_manager.git
cd 1724Rust_project_decentralized_password_manager

