# Project Proposal: Decentralized Password Manager

## Team Members

- **Siting Li** - 1005046335 - lisitin1
- **Zifan Meng** - 12345678 - lalalala

## Motivation

As the need for digital security grows, the flaws and risks of centralized password managers have become clear. Although centralized systems are useful, hackers like them because they only need one place to go wrong for the whole system to stop working. Public hacks have shown how weak these systems are, putting the private information of millions of users at risk and lowering trust in centralized solutions.

Our goal with this project is to make a safe, open password manager that can be used instead of the ones that are currently available. Using a peer-to-peer design lets us spread storage across many devices, so we don't have to rely on a central server. This makes data breaches much less likely. Users have full control over their data with this decentralized method, and sensitive data is never saved in plaintext on any server, which protects privacy.

This job is perfect for Rust because it has memory safety, zero-cost abstractions, and a strong asynchronous ecosystem - all of which are needed to make systems that are safe and work well. Our team can make a fast, decentralized password manager that puts user privacy first using Rust's strong tools for networking and cryptography. In addition, there aren't many decentralized password management solutions in the Rust environment. This project will fill that gap and may help the community as a whole by offering a new, more private way to protect passwords.

## Objective and Key Features

### Objective
The primary objective of this project is to design and implement a secure, decentralized password manager that ensures data privacy through a zero-knowledge architecture and end-to-end encryption. This password manager will utilize peer-to-peer (P2P) networking for device syncing, with all sensitive information encrypted before transmission.

### Key Features
1. **End-to-End Encryption**: Strong cryptographic methods will be used to hide user passwords on each device, which will protect data privacy across the network. Sodiumoxide, which is a Rust wrapper around NaCl, a widely used cryptography tool, will be used for encryption.
  
2. **Peer-to-Peer Syncing**: With the `libp2p` library, devices can talk to each other over a P2P network to keep their secret information in sync. Users won't have to rely on a central computer because they can get to their passwords from any device connected to the network.

3. **Command-Line Interface (CLI)**: A CLI will make it easy for users to add, retrieve, and handle passwords. The commands will be easy to understand and follow, just like in the command line, so both coders and power users will be able to use the tool.

4. **Zero-Knowledge Architecture**: The password manager will be made using zero-knowledge principles, which means that no one else will be able to see or record the user's plaintext passwords. Each device will only keep the encrypted version of the data. This way, even if the data is stolen, no useful information can be gotten from it.

## Tentative Plan
Our team will split up the work into three key areas so that we can finish the project on time: peer-to-peer networking, encryption and security, and integrating the user interface and database. Here is how our plan is broken down:

1. **Peer-to-Peer Networking**:
   - **Assigned Team Member**: Siting Li
   - **Responsibilities**: 
      - Implement decentralized syncing functionality using `libp2p`.
      - Configure peer discovery, secure communication, and file transfer protocols.
      - Ensure synchronization of encrypted password data across devices, addressing issues like data consistency and conflict resolution.

2. **Encryption and Security**:
   - **Assigned Team Member**: Zifan Meng
   - **Responsibilities**:
      - Set up end-to-end encryption using Sodiumoxide for encrypting passwords on the client side.
      - Implement secure hashing for the master password with Argon2.
      - Conduct security reviews to ensure that data remains secure both in transit and at rest.

3. **User Interface and Database Integration**:
   - **Shared Responsibility**: Both team members
   - **Responsibilities**:
      - Develop a command-line interface (CLI) using Clap or Structopt for managing passwords.
      - Implement password storage using Diesel and SQLite, ensuring the database only holds encrypted data.
      - Integrate CLI with database operations, allowing users to easily add, retrieve, and delete passwords from a secure local database.
      - Tests covering encryption, database operations, and peer-to-peer syncing.

### Timeline

1. **Weeks 1**: Set up project environment, dependencies, and preliminary modules. Develop initial CLI and basic database storage.
2. **Weeks 2**: Implement encryption functions, and set up peer-to-peer communication. Begin integration between modules.
3. **Weeks 3-4**: Complete full integration, refine CLI, add error handling, and conduct unit and integration testing.
4. **Week 5**: Finalize documentation and prepare a video demo.

## Conclusion
This project aims to fill a notable gap in the Rust ecosystem by providing a decentralized, secure, and zero-knowledge password manager. By dividing tasks among the team and focusing on specific objectives, we are confident in delivering a robust application within the project timeline. The combination of Rust¡¯s safety guarantees, peer-to-peer networking, and encryption libraries offers a unique opportunity to create a highly secure and private solution for password management.
