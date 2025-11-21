<div align="center">

<pre>
                                                         
▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄   ▄▄▄  ▄▄   ▄▄ ██████ ▄▄  ▄▄ ▄▄ ▄▄ 
▀▀▀▄▄▄ ██▄██ ██▀██ ██▀██ ██▀██ ██ ▄ ██ ██▄▄   ███▄██ ██▄██ 
█████▀ ██ ██ ██▀██ ████▀ ▀███▀  ▀█▀█▀  ██▄▄▄▄ ██ ▀██  ▀█▀  
                                                           
██████ ▄▄  ▄▄  ▄▄▄▄ ▄▄▄▄  ▄▄ ▄▄ ▄▄▄▄ ▄▄▄▄▄▄ ▄▄  ▄▄▄  ▄▄  ▄▄ 
██▄▄   ███▄██ ██▀▀▀ ██▄█▄ ▀███▀ ██▄█▀  ██   ██ ██▀██ ███▄██ 
██▄▄▄▄ ██ ▀██ ▀████ ██ ██   █   ██     ██   ██ ▀███▀ ██ ▀██ 
                                                            
v1.0.0
</pre>

[![Rust](https://img.shields.io/badge/Built_with-Rust-B94700.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/badge/Release-v1.0.0-371F54.svg?style=for-the-badge)](https://github.com/shadowdevforge/ShadowEnv-Encryption/releases)
[![Crypto](https://img.shields.io/badge/Encryption-XChaCha20--Poly1305-success?style=for-the-badge)](https://docs.rs/chacha20poly1305/latest/chacha20poly1305/)

**Production-grade, zero-knowledge directory encryption system.**  
*Pack, Compress, and Securitize your digital assets.*

[Installation](#installation) • [Usage](#usage) • [Troubleshooting](#-troubleshooting) • [Cryptography](#cryptography)

</div>

---

## 📔 Overview

**ShadowEnv** is a high-performance CLI tool designed to secure entire directory structures into a single, encrypted artifact (`.shadow` file). Unlike standard file encryption tools, ShadowEnv handles the entire pipeline: **Archival (Tar) -> Compression (Zstd) -> Encryption (AEAD)**.

It features a hybrid interface:
1.  **Interactive TUI**: A beautiful, Nerd-Font powered "Hacker Mode" for manual usage.
2.  **Scriptable CLI**: Standard flags for automated backups and cron jobs.

## 📷 Screenshot

<pre>
<img width="873" height="398" alt="Screenshot From 2025-11-22 01-38-03" src="https://github.com/user-attachments/assets/ea29a23f-ce5d-4337-b46f-2ea82ba07a34" />
</pre>

## ⚡ Features

*   **⚔️ Military-Grade Crypto**: Uses **XChaCha20-Poly1305** (Extended Nonce) for encryption and **Argon2id** for password hashing.
*   **📦 Smart Archival**: Recursively packs directories while intelligently ignoring existing `.shadow` files to prevent recursion loops.
*   **🚀 Zstd Compression**: Compresses data *before* encryption to maximize space efficiency.
*   **🎨 Aesthetic UI**: Features a **Catppuccin Macchiato** themed terminal interface with Nerd Font icons.
*   **🛡️ Memory Safe**: Written in pure Rust. No segfaults, no buffer overflows.
*   **🐧 Zero-Knowledge**: The password is never stored. It is mathematically impossible to recover data without the passphrase.

## 📥 Installation

### Prerequisites
**ShadowEnv uses Nerd Fonts** for its interface. Ensure your terminal is using a patched font (e.g., [JetBrainsMono NF](https://www.nerdfonts.com/font-downloads), Hack NF) or the icons will not render correctly.

### From Source
```bash
# Clone the repository
git clone https://github.com/shadowdevforge/ShadowEnv-Encryption.git

# Navigate to directory
cd ShadowEnv-Encryption

# Build and Install
cargo install --path .
```

## 🕹️ Usage

### 1. Interactive Mode [RECOMMENDED]
Simply run the command without arguments to enter the interactive wizard.

```bash
shadowenv
```
*Follow the prompts to select folders, input passwords, and verify paths.*

### 2. CLI Mode (Automation)
Perfect for scripts and backups.

**Encrypt a folder:**
```bash
# Syntax: shadowenv encrypt <FOLDER_PATH>
shadowenv encrypt ~/projects/my_secrets
```
*Creates `~/projects/my_secrets/my_secrets.shadow`*

**Decrypt a file:**
```bash
# Syntax: shadowenv decrypt <FILE_PATH>
shadowenv decrypt ~/projects/my_secrets/my_secrets.shadow
```
*Restores contents to `~/projects/my_secrets/my_secrets_restored/`*

---

## 🔧 Troubleshooting

### "Command not found: shadowenv"
If you successfully ran `cargo install` but the command doesn't run, your Rust binary directory is likely not in your system `PATH`.

**Fix (Choose your shell):**

**Bash / Zsh** (Linux/macOS):
Add this to your `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Fish Shell**:
Run this command once (it persists automatically):
```fish
fish_add_path $HOME/.cargo/bin
```

**PowerShell** (Windows):
Run this to add it to your current session:
```powershell
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```
*To make it permanent on Windows, search for "Edit the system environment variables" in the Start Menu, click "Environment Variables", select "Path" under User variables, and add `%USERPROFILE%\.cargo\bin`.*

---

### "Weird Squares / Broken Icons" ( )
ShadowEnv relies on **Nerd Fonts** to render the UI icons (locks, keys, folders). If you see squares, question marks, or weird glyphs:

**Fix:**
1.  Download a patched font like [JetBrainsMono Nerd Font](https://www.nerdfonts.com/font-downloads) or [Hack Nerd Font](https://github.com/ryanoasis/nerd-fonts).
2.  Install the font on your system.
3.  **Crucial Step:** Open your Terminal Settings (Preferences) and change the display font to the Nerd Font you just installed (e.g., "JetBrainsMono NF").

## 🔐 Cryptography & Architecture

ShadowEnv follows a strict "Encrypt-then-MAC" authenticated encryption pipeline.

### The Pipeline
1.  **Input**: Folder structure is read.
2.  **Archive**: `tar` bundles the files into a single stream.
3.  **Compression**: `zstd` compresses the stream (Level 0-3 default).
4.  **Key Derivation**: 
    *   Algorithm: **Argon2id** (Memory-hard).
    *   Salt: Random 16-byte salt generated per file.
5.  **Encryption**:
    *   Cipher: **XChaCha20-Poly1305**.
    *   Nonce: 24-byte random nonce (Safe against collisions).
6.  **Output**: `[Header] [Salt] [Nonce] [Ciphertext]`.

### Security Model
*   **Integrity**: Poly1305 ensures that if a bit is flipped in the `.shadow` file (corruption or tampering), decryption fails instantly.
*   **Confidentiality**: Without the password, the file is statistically indistinguishable from random noise.
*   **Hardness**: Argon2id prevents GPU/ASIC brute-force attacks.

## ⚠️ Disclaimer

This tool is provided "as is". While it uses industry-standard cryptographic primitives (RustCrypto), **if you forget your password, your data is lost forever.** There is no backdoor, no recovery key, and no "I forgot my password" button.

## 🤝 Contributing

Contributions are welcome!
1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

<div align="center">
  
  ___ShadowEnv Encryption forged___

</div>
