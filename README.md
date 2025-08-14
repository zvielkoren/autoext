# Auto Extension Switcher (Rust)

## Project Overview
**Auto Extension Switcher** is a small Rust tool that automatically renames newly created files in a folder based on the user-selected file format.  
This project helps practice Rust skills including filesystem monitoring, user input, error handling, and colored terminal output.

---

## MVP Features
- Watch a folder for **new file creations**.
- Prompt the user to choose the desired **file format** (Rust, Python, HTML, JavaScript, C++).
- Automatically **rename the file** with the selected extension.
- Use **colored output** for better UX.

---

## Flow

1. User runs the program:
   ```bash
   autoext ./watched
    ```

2. Create a new file in `./watched` folder.
3. The program detects the file creation.
4. Prompts the user:

   ```
   Which format do you want? (Rust, Python, HTML, JS, C++)
   ```
5. User selects a format.
6. File is renamed automatically with the chosen extension.
7. MVP exits after processing the first file.

---

## Suggested Crates

* **notify** → Watch a folder for new files.
* **dialoguer** → Nice CLI prompts for user input.
* **anyhow** → Simplified error handling.
* **colored** → Colored terminal output.

---

## Rust Project Setup

### Cargo.toml

```toml
[package]
name = "autoext"
version = "0.1.0"
edition = "2021"

[dependencies]
dialoguer = "0.11.0"
notify = "8.2.0"
anyhow = "1.0.99"
colored = "3.0.0"
```

---

### Usage Instructions

1. Build the project:

   ```bash
   cargo build --release
   ```
2. Run the program:

   ```bash
   ./target/release/autoext ./watched
   ```
3. Create a new file in the folder you are watching:

   ```bash
   touch ./watched/new_file
   ```
4. Follow the terminal prompt to select the file format.
5. The file will be renamed automatically:

   ```
   Renamed to: new_file.rs
   ```

---

## Future Enhancements

* Continuous background monitoring (not exit after one file).
* Configurable custom extension mapping.
* File templates for each language.
* Multi-file batch handling.
* Cross-platform packaging for Windows, macOS, Linux.

---

## MVP Success Criteria

* Detects a new file in the watched folder.
* Prompts user for file format.
* Renames file correctly.
* Works with colorized terminal output.
* Handles errors gracefully.

---
