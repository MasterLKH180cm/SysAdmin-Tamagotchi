# 🐾 SysAdmin Tamagotchi

> **Your system monitor just got adorable.** Turn boring Windows Task Manager into a living, breathing desktop pet that reflects your PC's health!

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)

## 🎮 What is This?

SysAdmin Tamagotchi is a **gamified system monitor** that lives in your Windows system tray. Instead of staring at boring graphs and percentages, you get a cute digital pet whose mood, health, and appearance change based on your computer's performance.

Your pet is **not just cute—it's functional**. When your RAM is maxed out, CPU is overheating, or temp files are piling up, your pet will show visible signs of distress. Click on it to perform quick maintenance actions!

## ✨ Features

### 🍔 Hunger = RAM Usage
- **Low RAM usage**: Happy, energetic pet
- **Medium RAM usage**: Pet gets chubby and slower
- **High RAM usage**: Pet becomes overstuffed and can barely move
- **Action**: One-click memory release to feed your pet properly

### 🔥 Mood = CPU Temperature/Load
- **Normal CPU**: Content, calm pet
- **Medium load**: Pet starts sweating
- **High temperature**: Pet gets angry and catches fire!
- **Action**: Shows warning for overheating conditions

### 🧹 Cleanliness = Junk Files
- **Clean system**: Sparkling pet environment
- **Some junk**: A few flies buzzing around
- **Lots of junk**: Swarm of flies and dirty surroundings
- **Action**: One-click cleanup of temp files and recycle bin

### 🛠️ Quick Actions
- **Right-click menu**:
  - 🗑️ Clear temp files
  - 📊 Open Task Manager
  - 🔄 Free up RAM
  - 📈 View detailed stats
  - ⚙️ Settings
  - ❌ Exit

## 🚀 Installation

### Prerequisites
- Windows 10 or later
- Rust 1.70+ (for building from source)

### Download Binary (Recommended)
1. Go to [Releases](https://github.com/yourusername/SysAdmin-Tamagotchi/releases)
2. Download the latest `SysAdmin-Tamagotchi.exe`
3. Run the executable
4. Your pet will appear in the system tray!

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/SysAdmin-Tamagotchi.git
cd SysAdmin-Tamagotchi

# Build release version
cargo build --release

# Run
./target/release/SysAdmin-Tamagotchi.exe
```

## 🎨 Usage

1. **Launch the app** - The pet appears in your system tray
2. **Watch your pet** - It automatically reflects your system status
3. **Click the pet** - Opens quick action menu
4. **Hover for details** - See exact RAM/CPU/Storage stats
5. **Keep your pet happy** - Maintain your system health!

## 🧰 Technical Details

### System Monitoring
- **RAM Usage**: Real-time memory consumption tracking
- **CPU Temperature**: Hardware sensor reading via WMI
- **CPU Load**: Process usage monitoring
- **Junk Files**: Scans `%TEMP%`, Recycle Bin, and common cache locations

### Technologies Used
- **Rust**: Core application logic
- **windows-rs**: Windows API bindings
- **sysinfo**: System information gathering
- **tray-icon**: System tray integration
- **wmi-rs**: Windows Management Instrumentation access

## 📊 Status Thresholds

| Status | RAM Usage | CPU Temp | Junk Files |
|--------|-----------|----------|------------|
| 😊 Happy | < 60% | < 65°C | < 500MB |
| 😐 Okay | 60-80% | 65-80°C | 500MB-2GB |
| 😰 Stressed | 80-90% | 80-90°C | 2GB-5GB |
| 🔥 Critical | > 90% | > 90°C | > 5GB |

## ⚙️ Configuration

Create a `config.toml` in the same directory as the executable:

```toml
[thresholds]
ram_warning = 75      # RAM usage percentage
ram_critical = 90
cpu_temp_warning = 75  # CPU temperature in Celsius
cpu_temp_critical = 85
junk_warning = 1024    # Junk files in MB
junk_critical = 3072

[appearance]
update_interval = 5    # Update frequency in seconds
animation_speed = 2    # Pet animation speed multiplier

[actions]
auto_cleanup = false   # Auto-clean when critical
show_notifications = true
```

## 🎯 Roadmap

- [ ] Multiple pet skins/themes
- [ ] Achievement system
- [ ] Historical performance graphs
- [ ] Custom action scripts
- [ ] Multi-language support
- [ ] Cross-platform support (Linux, macOS)
- [ ] Pet evolution based on system uptime
- [ ] Integration with other monitoring tools

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup
```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the classic Tamagotchi virtual pets
- System monitoring concepts from various task manager alternatives
- Rust community for excellent crates and support

## 📧 Contact

- **Issues**: [GitHub Issues](https://github.com/yourusername/SysAdmin-Tamagotchi/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/SysAdmin-Tamagotchi/discussions)

---

**Made with 🦀 and ❤️ by developers who think system monitoring should be fun!**

*Keep your system healthy, keep your pet happy!* 🐾✨