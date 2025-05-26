# 🛡️ clog — Cryptographically Secure Daily Journal/Diary 

`clog` is a Rust crate for safely creating, storing, and updating **daily notes or any content** using **cryptographically secure methods**. All files and metadata are stored in a **single encrypted `.clog(or custom file extension)` file**, making your private thoughts or sensitive content both secure and portable. Without the correct password, **no content can be accessed**.

This crate is ideal for **diary writing**, **private note-taking**, or **storing content**, where privacy and tamper-resistance are essential.

---

## ✨ Features

- 🔐 End-to-end password-based encryption
- 📁 Entries organized in auto-created **date-based folders** (`dd/mm/yyyy`)
- 📝 Only entries inside **today's folder can be edited**
- 📄 All content (notes + metadata) stored in **a single encrypted file**
- 👥 Multi-user support
- 🧾 Clean JSON metadata export for backup or syncing

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clog_rs = "0.0.1"
```

---

## 🔧 Exposed API

### ➕ Add New User

```rust
add_new_user(password: &str, clogfile_path: &str)
```

Creates a new encrypted `.clog(or custom file extension)` file with a user.

---

### 📁 Add a Folder

```rust
add_folder(clogfile_path: &str, password: &str)
```

Creates a new folder for the current date. Automatically handled internally.

---

### 📄 Add a Note

```rust
add_file(password: &str, clogfile_path: &str, filename: &str, file_content: &str)
```

Adds a note to today’s folder (`dd/mm/yyyy`). Creates the folder if missing.

---

### ✏️ Edit a Note

```rust
update_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
)
```

Replaces content of a note inside **today's folder only**. Older notes are read-only.

---

### 🔓 Read a Note

```rust
get_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
) -> String
```

Decrypts and returns note content if the password is correct.

---

### 🧠 Get JSON Metadata

```rust
get_json_metadata(password: &str, clogfile_path: &str) -> String
```

Returns stripped metadata as a clean JSON string.

---

## 🧱 Metadata Structure

Internally, metadata is securely stored in the `.clog(or custom file extension)` file. It looks like this (JSON):

```json
{
  "folders": {
    "25/05/2025": {
      "morning-thoughts.txt": {
        "created_at": "08:15:02 AM"
      },
      "evening-reflection.md": {
        "created_at": "08:55:42 PM"
      }
    },
    "24/05/2025": {
      "goals.txt": {
        "created_at": "03:31:12 PM"
      }
    }
  },
  "created_at": "24/05/2025"
}
```

- `folders` — contains all journal folders by date.
- Each note includes `created_at` in `"%I:%M:%S %p"` format (e.g., `08:15:02 AM`).
- Top-level `created_at` marks the file's creation date.

---

## 🔐 Security Model

- Uses AES encryption from trusted Rust crypto libraries.
- All content — notes + metadata — encrypted in **one file**.
- Without password, nothing is accessible.
- Only **today's entries are editable** — older entries are locked.

---

## 🚀 Example Usage

```rust
use clog_rs::*;

let clog_path = "my_journal.clog";
let password = "super_secure_password";

// Step 1: Create your secure journal
add_new_user(password, clog_path);

// Step 2: Add a new entry
add_file(password, clog_path, "reflection.txt", "Today I learned something new...");

// Step 3: Read it later
let content = get_file_content(password, clog_path, "reflection.txt", "25/05/2025");
println!("Decrypted entry: {}", content);

// Step 4: View metadata
let metadata = get_json_metadata(password, clog_path);
println!("Metadata: {}", metadata);
```

---

## 📄 License

MIT © 2025 Deep Gajjar

---

## 🤝 Contributions

PRs and suggestions are welcome! Let's make encrypted journaling elegant and secure.
