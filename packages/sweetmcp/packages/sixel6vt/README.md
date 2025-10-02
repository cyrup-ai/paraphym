# sixel6vt

Terminal emulator with sixel graphics support, built on Rio terminal.

## Quick Start

### Prerequisites
- Rust toolchain
- Git
- `just` command runner (optional but recommended)

### Setup

```bash
# Clone Rio and convert to library
just setup

# Or manually:
cargo run -p xtask -- setup
```

### Build

```bash
# Build
just build

# Or manually:
cargo build -p sixel6vt
```

### Run

```bash
just run

# Or manually:
cargo run -p sixel6vt
```

## Project Structure

```
sixel6vt/
├── Cargo.toml              # Workspace definition
├── justfile                # Task automation
├── xtask/                  # Build automation
│   └── src/main.rs         # Setup script (clones Rio, converts to lib)
├── packages/
│   └── sixel6vt/           # Main application
│       ├── Cargo.toml
│       └── src/main.rs
├── vendor/                 # Created by setup
│   └── rio/                # Rio terminal (cloned from GitHub)
└── tmp/                    # Reference code only (not used in build)
    └── rio/                # Read-only Rio source for reference
```

## Workflow

1. **First time setup**: `just setup`
   - Clones Rio from GitHub to `vendor/rio`
   - Creates `lib.rs` in rioterm with module exports
   - Adds `[lib]` section to rioterm's Cargo.toml
   - Verifies library builds

2. **Development**: `just build` or `just run`
   - Uses Rio as a library via path dependencies
   - Builds sixel6vt application

3. **Clean start**: `just clean-all && just setup`
   - Removes vendor directory
   - Re-clones and sets up Rio

## Available Commands

```bash
just                # Show all commands
just setup          # Initial Rio setup
just build          # Build project
just release        # Build release version
just run            # Run application
just init           # Full workflow (setup + build)
just clean          # Remove vendor directory
just clean-all      # Clean everything including cargo artifacts
just check          # Run cargo check
just test           # Run tests
just fmt            # Format code
```

## Development Notes

- **tmp/rio**: Reference code only, read-only, not used in builds
- **vendor/rio**: Active build dependency, modified by xtask
- **xtask**: Separate workspace member, no Rio dependencies
- **packages/sixel6vt**: Main app, depends on Rio via path

## Troubleshooting

**Problem**: `vendor/rio` not found
**Solution**: Run `just setup`

**Problem**: Build fails with workspace errors
**Solution**: Ensure you're in the workspace root and `vendor/rio` exists

**Problem**: Want to update Rio
**Solution**: Delete `vendor/rio` and run `just setup` again
