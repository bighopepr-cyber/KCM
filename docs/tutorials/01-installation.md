# Tutorial 01: Installation

## Objective

Install KCM and verify the installation works correctly.

## Prerequisites

- Rust 1.85+ (install via rustup)
- Git

## Steps

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Should show 1.85+
```

### Step 2: Clone KCM

```bash
git clone https://github.com/kcm-project/KCM.git
cd KCM
```

### Step 3: Build KCM

```bash
cargo build --release
```

This compiles all 13 crates and produces the server binary.

### Step 4: Verify Installation

```bash
# Run tests
cargo test --workspace

# Check version
./target/release/kcm-server --version
```

### Step 5: Run Benchmarks (Optional)

```bash
cargo bench --workspace
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `rustc: command not found` | Run `source $HOME/.cargo/env` |
| Build fails with memory error | Increase swap or use `--release` profile |
| Tests fail | Check Rust version >= 1.85 |

## Next Steps

- Tutorial 02: Create your first database
