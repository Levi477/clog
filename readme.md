# 🛡️ clog — Cryptographically Secure Daily Journal/Diary API

`clog` is a Rust crate designed to help you securely store daily notes, thoughts, or any sensitive content — all inside a **single encrypted `.clog` file**. Every note is organized in a **virtual folder-file structure**, mimicking a traditional file system — but with encryption and portability in mind.

Without the correct password, nothing can be accessed — not even metadata.

This crate is ideal for **journaling**, **private notes**, and **tamper-proof content storage**.

---

## 📦 Application

> Try the terminal-based daily diary built with `clog_rs`:  
> 👉 [clog-tui v1.3.0](https://github.com/Levi477/clog-tui/releases/tag/v1.3.0)

---

## ✨ Features

- 🔐 End-to-end AES encryption (password-based)
- 📁 Entries auto-organized by **virtual date-based folders** (e.g. `25/05/2025`)
- 📝 Only entries from **today can be edited** (others are read-only)
- 📄 **All notes and metadata stored in a single `.clog` file**
- 👥 Multi-user support (password-protected)
- 🧾 Export clean JSON metadata for syncing or backups

---

## 📁 How Storage Works

Instead of using actual folders and files on disk, `clog` creates a **virtual file system** inside a `.clog` file.

### 🔍 Example:

Suppose you write two entries on different dates:

- On `25/05/2025`: `morning-thoughts` and `evening-reflection`
- On `24/05/2025`: `goals`

All of this is stored inside `my_journal.clog` like so:

```json
{
  "folders": {
    "25/05/2025": {
      "morning-thoughts": {
        "created_at": "08:15:02 AM"
      },
      "evening-reflection": {
        "created_at": "08:55:42 PM"
      }
    },
    "24/05/2025": {
      "goals": {
        "created_at": "03:31:12 PM"
      }
    }
  },
  "created_at": "24/05/2025"
}
```

📌 **Note**: All this lives inside **a single `.clog` file** — portable, encrypted, and compact.

---

## 🚀 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
clog_rs = "1.0.1"
```

or use 

```bash
cargo add clog_rs
```

---

## 🛠️ API Overview

### ➕ `add_new_user`

```rust
add_new_user(password: &str, clogfile_path: &str)
```

Creates a new encrypted `.clog` file and initializes the metadata.

---

### 📝 `add_file`

```rust
add_file(password: &str, clogfile_path: &str, filename: &str, file_content: &str)
```

Adds a file to **today’s folder** (auto-created if missing).

---

### ✏️ `update_file_content`

```rust
update_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
)
```

Edits a file only if it's in **today's folder**. Older notes are immutable.

---

### 🔓 `get_file_content`

```rust
get_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
) -> String
```

Decrypts and returns content if password matches.

---

### `get_json_metadata`

```rust
get_json_metadata(password: &str, clogfile_path: &str) -> String
```

Returns metadata (folder + file structure) as a JSON string.

---

## 🔐 Security Model

- All data is encrypted using AES (via well-audited Rust crypto libraries)
- No plaintext or filesystem traces — everything is embedded in `.clog`
- Zero access without password
- Only today's entries can be changed — a form of **cryptographic journaling discipline**

---

## ⚡ Example Usage

```rust
use clog_rs::*;

let clog_path = "my_journal.clog";
let password = "super_secure_password";

// Step 1: Create a new encrypted journal
add_new_user(password, clog_path);

// Step 2: Add today's note
add_file(password, clog_path, "something", "Today I learned something new...");

// Step 3: Read it back
let content = get_file_content(password, clog_path, "something", "25/05/2025");
println!("Decrypted entry: {}", content);

// Step 4: Get metadata
let metadata = get_json_metadata(password, clog_path);
println!("Journal structure: {}", metadata);
```

---

## 📄 License

MIT © 2025 Deep Gajjar

---

## 🤝 Contributions

PRs, issues, and feedback are welcome.
