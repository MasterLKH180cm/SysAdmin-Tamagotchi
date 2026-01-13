# SysAdmin Tamagotchi - Developer Guide

## Project Structure

```
SysAdmin-Tamagotchi/
├── src-tauri/              # Tauri backend (Rust)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── lib.rs          # Library exports for testing
│   │   ├── commands.rs     # Tauri commands (get_metrics, cleanup_temp, etc.)
│   │   ├── monitor.rs      # System monitoring logic
│   │   ├── pet.rs          # Pet state logic
│   │   └── poller.rs       # Background metric polling
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── build.rs            # Build script
├── ui/                     # Svelte frontend
│   ├── src/
│   │   ├── main.js         # Entry point
│   │   ├── App.svelte      # Main app component
│   │   └── components/
│   │       ├── Pet.svelte           # Pet display with animations
│   │       ├── ParticleEffect.svelte # Canvas particle effects
│   │       ├── StatsTooltip.svelte  # Metrics tooltip
│   │       └── ActionMenu.svelte    # Action buttons
│   ├── package.json        # Node.js dependencies
│   ├── vite.config.js      # Vite configuration
│   └── index.html          # HTML entry point
├── tests/                  # Integration tests
│   ├── test_monitor.rs     # Monitor tests
│   ├── test_pet.rs         # Pet state tests
│   └── manual_test_checklist.md
└── README.md
```

## Prerequisites

### Required Software

1. **Rust** (latest stable)
   ```bash
   # Download from: https://rustup.rs/
   # Or install via:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18 or higher)
   ```bash
   # Download from: https://nodejs.org/
   ```

3. **Visual Studio C++ Build Tools** (Windows only)
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Desktop development with C++" workload

### Verify Installation

```bash
# Check Rust
cargo --version
rustc --version

# Check Node.js
node --version
npm --version
```

## Development Setup

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/SysAdmin-Tamagotchi.git
cd SysAdmin-Tamagotchi
```

### 2. Install Frontend Dependencies

```bash
cd ui
npm install
cd ..
```

### 3. Run in Development Mode

**Option A: Using npm scripts (Recommended)**

```bash
# From project root
cd src-tauri
cargo tauri dev
```

This will:
- Start Vite dev server on http://localhost:5173
- Launch Tauri window with hot-reload enabled
- Enable Rust and Svelte hot-reloading

**Option B: Manual (two terminals)**

Terminal 1 (Frontend):
```bash
cd ui
npm run dev
```

Terminal 2 (Backend):
```bash
cd src-tauri
cargo tauri dev
```

### 4. Development Workflow

- **Edit Svelte files** in `ui/src/` - Hot reloads automatically
- **Edit Rust files** in `src-tauri/src/` - Triggers recompilation
- **View console logs** - Press F12 in Tauri window

## Testing

### Automated Tests (Rust)

Run all backend tests:

```bash
cd src-tauri
cargo test --verbose
```

Run specific test file:

```bash
cd src-tauri
cargo test --test test_monitor
cargo test --test test_pet
```

Run with output:

```bash
cd src-tauri
cargo test -- --nocapture
```

### Manual UI Testing

Follow the checklist in `tests/manual_test_checklist.md`

### Code Quality Checks

```bash
cd src-tauri

# Check for compilation errors
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Building for Production

### Create Release Build

```bash
cd src-tauri
cargo tauri build
```

This creates:
- **Installer**: `src-tauri/target/release/bundle/nsis/SysAdmin Tamagotchi_0.1.0_x64-setup.exe`
- **Portable**: `src-tauri/target/release/sysadmin-tamagotchi.exe`

### Build Artifacts

```
src-tauri/target/release/
├── bundle/
│   └── nsis/
│       └── SysAdmin Tamagotchi_0.1.0_x64-setup.exe  (Installer)
└── sysadmin-tamagotchi.exe                          (Portable)
```

### Optimizations Enabled

The release build uses these optimizations (in `Cargo.toml`):

```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization
strip = true          # Remove debug symbols
```

## Architecture Overview

### Backend (Rust)

**monitor.rs**
- Wraps `sysinfo` crate
- Collects RAM, CPU, Disk metrics
- Implements 30-second temporal smoothing for CPU
- Updated thresholds: 70/85/95%

**pet.rs**
- Defines `PetState` enum (Happy, Okay, Stressed, Critical)
- Maps metrics to pet states
- Emoji representation: 😊 😐 😰 🔥

**commands.rs**
- Tauri command handlers
- `get_metrics()` - Returns current metrics + pet state
- `cleanup_temp()` - Deletes temp files
- `get_pet_state()` - Returns current pet state

**poller.rs**
- Background tokio task
- Polls metrics every 5 seconds
- Emits `metrics-update` events to frontend

### Frontend (Svelte)

**App.svelte**
- Main container
- Manages state and events
- Listens to backend updates

**Pet.svelte**
- Displays emoji
- Handles animations (bounce, sway, breathe, shake)
- Emits hover/click events

**ParticleEffect.svelte**
- Canvas-based particle system
- Shows fire particles in Critical state
- Uses requestAnimationFrame for smooth animation

**StatsTooltip.svelte**
- Shows RAM/CPU/Disk metrics
- Appears on hover

**ActionMenu.svelte**
- Cleanup button
- Close button
- Semi-transparent overlay

## Configuration

### Window Settings (`tauri.conf.json`)

```json
{
  "windows": [{
    "width": 300,
    "height": 300,
    "resizable": false,
    "decorations": false,      // No title bar
    "transparent": true,       // Transparent background
    "alwaysOnTop": true,      // Stay on top
    "skipTaskbar": true       // Hide from taskbar
  }]
}
```

### Thresholds (Domain Expert Approved)

**RAM**: 70% / 85% / 95%
**CPU**: 70% / 85% / 95% (with 30s smoothing)
**Disk**: 5% / 10% / 20% of total disk

### Polling Interval

Default: 5 seconds (configurable in `poller.rs`)

## Troubleshooting

### Build Fails on Windows

**Error**: "link.exe not found"
- Install Visual Studio C++ Build Tools
- Restart terminal after installation

**Error**: "OpenSSL not found"
```bash
# Install via vcpkg or use system OpenSSL
cargo clean
cargo build
```

### Frontend Not Loading

**Error**: "Failed to connect to localhost:5173"
- Ensure Vite dev server is running
- Check port 5173 is not in use
- Try manual two-terminal approach

### Transparent Window Not Working

- Ensure Windows DWM (Desktop Window Manager) is enabled
- Check GPU drivers are up to date
- Transparency requires Windows 10+ with Aero enabled

### Permission Errors During Cleanup

- Expected on some systems
- Temp file cleanup requires write permissions
- Run as Administrator if needed

### Pet Not Updating

- Check console for errors (F12)
- Verify backend polling is running
- Check `metrics-update` events in console

## Performance Tips

### Reduce CPU Usage

Edit `poller.rs` to increase polling interval:
```rust
let mut interval = time::interval(Duration::from_secs(10)); // Changed from 5
```

### Reduce Memory Usage

Edit `monitor.rs` to limit CPU history:
```rust
max_history_size: 3,  // Changed from 6
```

### Optimize Build Size

Already optimized with `opt-level = "z"` and `lto = true`

Current build size: ~8-12MB (installer)

## Advanced Features

### Custom Emojis

Edit `pet.rs`:
```rust
pub fn emoji(&self) -> &str {
    match self {
        PetState::Happy => "🎉",  // Change emoji here
        // ...
    }
}
```

### Custom Thresholds

Edit `pet.rs` `update()` method:
```rust
let ram_status = Self::classify_metric(metrics.ram_percent, 60.0, 75.0, 90.0);
//                                                          ^     ^     ^
//                                                       warning stressed critical
```

### Window Position

Edit `tauri.conf.json`:
```json
{
  "x": 100,    // X coordinate
  "y": 100     // Y coordinate
}
```

## Contributing

### Code Style

- **Rust**: Use `cargo fmt` (rustfmt)
- **JavaScript/Svelte**: Use Prettier (optional)
- Follow existing patterns

### Commit Messages

Follow Conventional Commits:
```
feat: Add new pet animation
fix: Correct CPU threshold calculation
docs: Update README with new thresholds
test: Add integration test for cleanup
```

### Pull Request Process

1. Fork repository
2. Create feature branch
3. Make changes
4. Run tests (`cargo test`)
5. Run clippy (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Submit PR with description

## License

MIT License - see LICENSE file

## Credits

- Built with [Tauri](https://tauri.app/)
- Frontend: [Svelte](https://svelte.dev/)
- System monitoring: [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- Architecture designed per Domain Expert recommendations
- Implemented with AI assistance (Claude Sonnet 4.5)

## Support

For issues or questions:
1. Check `tests/manual_test_checklist.md`
2. Review Troubleshooting section above
3. Open GitHub issue with:
   - Windows version
   - Error message
   - Steps to reproduce

---

**Happy monitoring!** 😊
