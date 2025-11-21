# ShadowEnv Encryption

<div align="center">

![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)
![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge)
![Encryption](https://img.shields.io/badge/Encryption-XChaCha20--Poly1305-blueviolet?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

**Zero-Knowledge Recursive Directory Archival System**

</div>

---

## 💀 What is this?

**ShadowEnv** is a cryptographically secure archival tool written in Rust. It transforms directory structures into single, compressed, and encrypted `.shadow` artifacts.

Unlike standard zip tools with password protection, ShadowEnv uses **Authenticated Encryption (AEAD)**. It doesn't just lock the door; it melts the key.

### ✨ Key Features
*   **Military-Grade Crypto**: Uses **XChaCha20-Poly1305** (Extended Nonce) for encryption and **Argon2id** for key derivation.
*   **Compression First**: Compresses data with **Zstd** before encryption to maximize efficiency.
*   **Hacker Aesthetic**: Features a fully interactive **TUI (Text User Interface)** styled with the **Catppuccin Macchiato** palette and Nerd Fonts.
*   **Recursion Safety**: Smart path resolution prevents infinite loops or "snake eating tail" archival errors.
*   **Zero Trust**: The code does not store your password. It uses your password as the mathematical ingredient to derive the key. Without it, decryption is mathematically impossible.

---

## 🚀 Quick Start

### Prerequisites
1.  **Rust**: You need `cargo` installed. ([Get Rust](https://rustup.rs/))
2.  **Font**: A [Nerd Font](https://www.nerdfonts.com/) (e.g., Hack NF, JetBrainsMono NF) installed in your terminal to see icons.

### Installation

```bash
# Clone the repository
git clone https://github.com/shadowdevforge/ShadowEnv.git
cd ShadowEnv

# Build and Install
cargo install --path .
```

### Run It
To launch the interactive "Cyberpunk" Interface:
```bash
shadowenv
```

To use it in scripts (CI/CD or cron):
```bash
# Encrypt a folder
shadowenv encrypt ~/projects/secrets

# Decrypt a file
shadowenv decrypt ~/projects/secrets.shadow
```

---

## 🖥️ Visual Interface

ShadowEnv detects if you run it without arguments and launches a TUI.

```text
   _____ __               __               
  / ___// /_  ____ ______/ /___ _      __  
  \__ \/ __ \/ __ `/ __  / __ \ | /| / /   
 ___/ / / / / /_/ / /_/ / /_/ / |/ |/ /    
/____/_/ /_/\__,_/\__,_/\____/|__/|__/     
                                           
    v2.0 :: SHADOW ENV

Select Protocol ::
>  Encrypt Folder
   Decrypt Archive
   Exit

Target Configuration ::
? Path: ~/projects/markdown/ 
? Passphrase: *****
? Verify: *****

 Packing system: /home/user/projects/markdown
 Encrypting stream...

 SECURE OBJECT CREATED :: "/home/user/projects/markdown.shadow"
```

---

## 🔒 Security Architecture

### The Pipeline
1.  **Input**: Recursive directory walk (ignores existing `.shadow` files).
2.  **Archive**: Tarball creation (preserves permissions/structure).
3.  **Compression**: Zstd stream compression.
4.  **Derivation**: **Argon2id** hashes the user password with a random 22-byte Salt to generate a 32-byte Key.
5.  **Encryption**: **XChaCha20-Poly1305** encrypts the compressed stream using the derived Key and a random 24-byte Nonce.
6.  **Output**: A binary file containing `[MAGIC_HEADER] [SALT] [NONCE] [CIPHERTEXT]`.

### Why XChaCha20?
Standard ChaCha20 uses a 96-bit nonce, which risks collision (and total security failure) if enough files are encrypted with the same key. **XChaCha20** uses a 192-bit nonce, making random generation statistically safe forever.

### Why Argon2id?
It is memory-hard. It forces the attacker to use significant RAM to attempt a single password guess, neutralizing GPU/ASIC brute-force farms.

---

## 🛠️ Development

The codebase is modular and clean:

*   `src/main.rs`: TUI logic, path resolution, and CLI parsing (`clap` + `inquire`).
*   `src/crypto.rs`: Pure Rust implementation of the encryption/decryption pipeline.
*   `src/archive.rs`: Handling of the Tar/Zstd stream and recursion safety.

### Running Tests
```bash
cargo test
```

---

## ⚠️ Disclaimer

This tool is provided "as is". While it uses state-of-the-art cryptographic primitives, **if you forget your password, your data is lost forever.** There is no "Forgot Password" button. There is no backdoor. The math does not care.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

Project by **[shadowdevforge](https://github.com/shadowdevforge)**.
```
