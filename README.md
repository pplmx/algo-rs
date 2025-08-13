# 🦀 algo-rs — Rust LeetCode Solutions Template

A clean, organized Rust project template for solving LeetCode problems and algorithm challenges.

## ✨ Features

- **One problem, one file**: Each problem lives in `src/problems/` with number-based naming
- **Clean test separation**: All test code organized in dedicated `tests/` directory
- **Automated problem generation**: Use `scripts/new_problem.rs` to scaffold new problems with boilerplate
- **LeetCode-ready code**: Self-contained solutions that can be directly copied for submission

---

## 📂 Project Structure

```
algo-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── problems/
│       ├── mod.rs
│       └── p00001_two_sum.rs      # Example problem
├── tests/
│   └── test_p00001_two_sum.rs     # Corresponding test
└── scripts/
    └── new_problem.rs             # Problem generator script
```

---

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repo-url>
cd algo-rs
```

### 2. Run Example Tests

```bash
cargo test
```

### 3. Create New Problem

```bash
cargo run --bin new_problem 2 add_two_numbers
```

This generates:

- `src/problems/p00002_add_two_numbers.rs`
- `tests/test_p00002_add_two_numbers.rs`
- Updates `src/problems/mod.rs` automatically

### 4. Implement Solution

Edit `src/problems/p00002_add_two_numbers.rs`:

```rust
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(/* params */) -> /* return type */ {
        // Your implementation here
    }
}
```

### 5. Write Test Cases

Edit `tests/test_p00002_add_two_numbers.rs`:

```rust
use algo_rs::problems::p00002_add_two_numbers::Solution;

#[test]
fn test_example_case() {
    assert_eq!(Solution::add_two_numbers(/* args */), /* expected */);
}
```

---

## 📝 Naming Conventions

- **Problem files**: `pXXXXX_<snake_case_name>.rs`
    - Example: `00001 Two Sum` → `p00001_two_sum.rs`
- **Test files**: `test_pXXXXX_<snake_case_name>.rs`

This ensures files are **sortable, searchable, and easy to navigate**.

---

## 💡 LeetCode Submission

For LeetCode submissions, simply copy the solution block:

```rust
impl Solution {
    pub fn your_function(/* params */) -> /* return type */ {
        // Implementation
    }
}
```

The `pub struct Solution;` declaration is automatically included by LeetCode.

---

## 🛠 Development Tips

- Use `cargo test` to run all tests
- Use `cargo test p00001` to run specific problem tests
- Use `cargo check` for quick syntax validation
- Add `#[allow(dead_code)]` if you have unused helper functions

---

## 🎯 Roadmap

- [ ] Auto-fetch problem descriptions from LeetCode API
- [ ] Categorized problem organization (arrays, dynamic programming, graphs, etc.)
- [ ] Generate solution summary table in README
- [ ] Benchmark suite for performance testing
- [ ] Export solutions to different formats (Python, Java, etc.)

---

## 📜 License

MIT License

---

## 🤝 Contributing

Feel free to submit issues and enhancement requests! Pull requests are welcome.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
