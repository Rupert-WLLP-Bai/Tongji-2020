# Tongji-2020 Rust Migration

This project migrates ~110 C/C++ programming assignments from Tongji University to Rust.

## Project Structure

```
tongji-rust/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── console-tools/            # Cross-platform console manipulation library
│   ├── tier1-projects/           # Simple projects (3-b*, 4-b*)
│   ├── tier2-projects/           # Medium projects (5-b*, 6-b*)
│   ├── tier3-projects/           # Complex projects (7-b*, 8-b*, 9-b*)
│   ├── hanoi/                    # Hanoi Tower game (90-b1)
│   └── minesweeper/              # Minesweeper game (90-b2)
└── tauri-app/                    # GUI (Phase 5)
```

## Migration Phases

- **Phase 1**: Foundation Setup (console-tools + tier1-projects)
- **Phase 2**: Tier 2 Projects Migration
- **Phase 3**: Tier 3 Projects Migration
- **Phase 4**: Major Games Migration (Hanoi Tower + Minesweeper)
- **Phase 5**: Tauri + React GUI Integration

## Building

```bash
# Build all projects
cargo build --workspace

# Run a specific project
cargo run --bin 3-b10

# Run Hanoi Tower
cargo run -p hanoi

# Run Minesweeper
cargo run -p minesweeper
```

## Testing

```bash
# Test all crates
cargo test --workspace

# Test console-tools
cargo test -p console-tools
```

## Current Status

- [x] Phase 1: Foundation Setup (In Progress)
  - [x] Git branch created
  - [x] Workspace structure initialized
  - [ ] Console-tools library
  - [ ] First project migration (3-b10)
- [ ] Phase 2: Tier 2 Projects
- [ ] Phase 3: Tier 3 Projects
- [ ] Phase 4: Major Games
- [ ] Phase 5: Tauri GUI
