# Quick Start Guide

## For End Users

### Installation

1. **Download** the installer:
   - Get `SysAdmin Tamagotchi_0.1.0_x64-setup.exe` from releases

2. **Run** the installer:
   - Double-click the `.exe` file
   - Follow installation wizard
   - Choose installation location

3. **Launch** the application:
   - Find "SysAdmin Tamagotchi" in Start Menu
   - Or run from installation directory

### Usage

**Pet Display**
- Pet appears on your screen as an emoji
- Pet changes based on system health:
  - 😊 Happy - System is healthy
  - 😐 Okay - Minor resource usage
  - 😰 Stressed - High resource usage
  - 🔥 Critical - System overloaded

**View Stats**
- Hover over pet to see system metrics:
  - RAM usage percentage
  - CPU usage percentage
  - Disk junk size

**Actions**
- Click pet to open action menu
- Click "Clean Temp Files" to delete temporary files
- Click "Close" to close menu

**Move Pet**
- Click and drag to move pet around screen

### Uninstall

1. Open Windows Settings
2. Apps → Installed Apps
3. Find "SysAdmin Tamagotchi"
4. Click Uninstall

---

## For Developers

### Prerequisites

- Rust (latest stable): https://rustup.rs/
- Node.js (v18+): https://nodejs.org/
- Visual Studio C++ Build Tools (Windows)

### Quick Start

```bash
# Clone repository
git clone https://github.com/yourusername/SysAdmin-Tamagotchi.git
cd SysAdmin-Tamagotchi

# Install frontend dependencies
cd ui
npm install
cd ..

# Run in development mode
cd src-tauri
cargo tauri dev
```

### Run Tests

```bash
cd src-tauri
cargo test --verbose
```

### Build for Production

```bash
cd src-tauri
cargo tauri build
```

Output: `src-tauri/target/release/bundle/nsis/SysAdmin Tamagotchi_0.1.0_x64-setup.exe`

### Project Structure

```
├── src-tauri/          # Rust backend
│   └── src/
│       ├── main.rs     # Entry point
│       ├── monitor.rs  # System metrics
│       ├── pet.rs      # Pet state logic
│       ├── commands.rs # Tauri commands
│       └── poller.rs   # Background polling
├── ui/                 # Svelte frontend
│   └── src/
│       ├── App.svelte
│       └── components/
└── tests/              # Integration tests
```

### Key Technologies

- **Backend**: Rust + Tauri 2.x
- **Frontend**: Svelte 5 + Vite
- **System Monitoring**: sysinfo crate
- **Async Runtime**: Tokio

### Documentation

- Full developer guide: `DEVELOPER_GUIDE.md`
- Test checklist: `tests/manual_test_checklist.md`
- AI assistance notes: `CLAUDE.md`

---

## Troubleshooting

### "Pet window won't open"
- Check if another instance is running
- Ensure Windows version is 10 or 11
- Update GPU drivers

### "Transparent background not working"
- Enable Desktop Window Manager (DWM)
- Check Aero is enabled
- Update to latest Windows version

### "Cleanup doesn't delete files"
- Normal if you lack temp folder permissions
- Try running as Administrator
- Some files may be locked by other processes

### "High CPU usage"
- Increase polling interval in code (default: 5s)
- Check for runaway processes in Task Manager

---

## Getting Help

**Documentation**:
- README.md - Project overview
- DEVELOPER_GUIDE.md - Comprehensive development guide
- CLAUDE.md - AI assistance guidelines

**Issues**:
- Report bugs on GitHub Issues
- Include Windows version and error message
- Attach screenshots if UI-related

**Contributing**:
- Fork → Branch → Code → Test → PR
- Run `cargo fmt` and `cargo clippy` before submitting
- Follow existing code patterns

---

**Version**: 0.1.0
**Last Updated**: 2026-01-13
**Platform**: Windows 10/11
