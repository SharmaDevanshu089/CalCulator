# 🧮 Terminal Calculator in Rust

Welcome to **Devanshu's Terminal Calculator** – a beginner-friendly CLI calculator built in Rust 🦀. It supports basic arithmetic operations like addition, subtraction, multiplication, and division — all via a simple text interface!

> 💡 This project is part of my Rust 10x Developer Journey mentored by ChatGPT!

---

## 🚀 Features

- Interactive menu-based CLI
- User greeting with name input
- Support for large integers using `i128`
- Clean modular code (functions for each operation)
- Global username using `once_cell`
- Friendly messages and structured prompts

---

## 🛠️ Technologies Used

- Rust
- `once_cell` for global static memory
- `std::io` for input/output
- `match` and modular functions for control flow

---

## 📦 How to Run It Locally

### 1. Prerequisites

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

Or

```bash
sudo apt install rustc cargo
```

### 2. Clone This Repository

```bash
git clone https://github.com/sharmadevanshu089/rust-calculator.git
cd rust-calculator
```

### 3. Run the Calculator

```bash
cargo run
```

---

## 🧪 Example Usage

```bash
======  Hello There this is CalCulator created. =====
============= Please Enter your name ==============
> Devanshu

Hello, Devanshu!
What Operation Would you like to perform:
1 => Addition
2 => Subtraction
3 => Multiplication
4 => Division
5 => Quit
```

---

## 📁 Project Structure

```
src/
├── main.rs       # Main logic and function calls
Cargo.toml        # Dependencies (once_cell)
```

---

## 💡 Ideas for Improvement

* Add floating-point support (`f64`)
* Add power, square root, and modulus operations
* Improve input error handling
* Add colored terminal output (`colored` crate)
* Save operation history to a file

---

## 🧠 Author

Made with ❤️ by [Devanshu Sharma](https://github.com/sharmadevanshu089)

---

## 📜 License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

```
```
