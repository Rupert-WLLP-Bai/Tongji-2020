# Tongji-2020 Rust Migration

This project migrates ~110 C/C++ programming assignments from Tongji University to Rust.

## Project Structure

```
tongji-rust/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── console-tools/            # Cross-platform console manipulation library
│   ├── tier1-projects/           # Simple projects (3-b*, 4-b*, 5-b*)
│   ├── tier2-projects/           # Medium projects (6-b*)
│   ├── tier3-projects/           # Complex projects (7-b*, 8-b*, 9-b*)
│   ├── hanoi/                    # Hanoi Tower game (90-b1)
│   └── minesweeper/              # Minesweeper game (90-b2)
└── tauri-app/                    # GUI application with React frontend
```

## Migration Phases

- **Phase 1**: Foundation Setup (console-tools + tier1-projects) ✅
- **Phase 2**: Tier 2 Projects Migration ✅
- **Phase 3**: Tier 3 Projects Migration ✅
- **Phase 4**: Major Games Migration (Hanoi Tower + Minesweeper) ✅
- **Phase 5**: Tauri + React GUI Integration ✅

## Building

```bash
# Build all projects
cargo build --workspace

# Run a specific project (e.g., 3-b10)
cargo run --bin 3-b10

# Run Hanoi Tower (CLI version)
cargo run -p hanoi

# Run Minesweeper (CLI version)
cargo run -p minesweeper

# Run GUI application (requires Node.js and Tauri CLI)
cd tauri-app
bun install  # or npm install
bun run tauri dev  # or npm run tauri dev
```

## Testing

```bash
# Test all crates
cargo test --workspace

# Test console-tools
cargo test -p console-tools

# Test Hanoi Tower
cargo test -p hanoi

# Test Minesweeper
cargo test -p minesweeper
```

## Migration Status

### ✅ Completed (All Phases)

- **Phase 1**: Foundation Setup
  - ✅ Git branch created (`rust-migration`)
  - ✅ Workspace structure initialized
  - ✅ Console-tools library (cross-platform terminal control using crossterm)
  - ✅ Tier 1 projects migrated: **49 binaries**
    - 3-b series: 3-b2, 3-b3, 3-b4, 3-b5, 3-b6, 3-b7, 3-b8, 3-b9, 3-b10, 3-b12, 3-b13
    - 4-b series: 4-b1, 4-b3, 4-b4, 4-b7, 4-b8, 4-b9, 4-b11, 4-b12, 4-b13, 4-b17
    - 5-b series: 5-b1, 5-b3, 5-b6, 5-b7, 5-b8, 5-b9, 5-b10, 5-b11, 5-b13, 5-b14, 5-b15, 5-b16, 5-b17, 5-b18

- **Phase 2**: Tier 2 Projects
  - ✅ All 4 projects migrated: 6-b1, 6-b2, 6-b3, 6-b4

- **Phase 3**: Tier 3 Projects
  - ✅ All 11 projects migrated:
    - 7-b series: 7-b1, 7-b2
    - 8-b series: 8-b1, 8-b1-datagen, 8-b2
    - 9-b series: 9-b1, 9-b2, 9-b3-1, 9-b3-2, 9-b4-1, 9-b4-2

- **Phase 4**: Major Games
  - ✅ Hanoi Tower (90-b1): 748 lines of Rust code, fully functional CLI game
  - ✅ Minesweeper (90-b2): 1,187 lines of Rust code, fully functional CLI game with auto-solve

- **Phase 5**: Tauri GUI
  - ✅ Tauri + React + TypeScript application
  - ✅ React components for Hanoi Tower and Minesweeper
  - ✅ Modern web UI with game controls and visualization
  - ✅ Advanced auto-solve with set subtraction logic for Minesweeper

### 📊 Summary

- **Total Projects Migrated**: 64 projects
- **Console Library**: Fully functional cross-platform terminal control
- **Major Games**: 2 complete games with both CLI and GUI versions
- **GUI Application**: Modern Tauri + React interface

### 📝 Notes

- **Project1-4**: These correspond to early assignments (3-b2 ~ 3-b5) and have been successfully migrated to tier1-projects with comprehensive testing (50 unit tests, all passing)
- **Unmigrated Projects**: Remaining projects are mostly duplicate C versions or variants of already-migrated assignments
- **Cross-platform**: All code works on Windows, macOS, and Linux

## Features

### Console-Tools Library
- Cross-platform terminal control using `crossterm`
- Color management (16-color palette)
- Cursor positioning and visibility control
- Screen clearing and size detection
- Keyboard input handling (including arrow keys and function keys)

### Hanoi Tower Game
- Interactive CLI game with visual disk representation
- Configurable number of disks (1-10)
- Move validation and win detection
- Auto-solve feature with step-by-step animation
- GUI version with React controls

### Minesweeper Game
- Three difficulty levels (Beginner, Intermediate, Expert)
- Flood-fill algorithm for revealing empty cells
- Flag placement for marking mines
- Advanced auto-solve with:
  - Basic logical deduction
  - Set subtraction logic for complex patterns
  - Random guessing when logic is exhausted
- GUI version with React controls and visual feedback

## License

This is a migration of university coursework for educational purposes.
