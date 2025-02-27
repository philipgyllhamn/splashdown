# 🌊 Splashdown

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/yourusername/splashdown?style=social)](https://github.com/yourusername/splashdown)

> **Enhance your game streaming experience with beautiful loading screens**

Splashdown is a lightweight utility designed specifically for Apollo and Sunshine game streaming environments. It displays an elegant splash screen while your game loads in the background, creating a seamless and professional streaming experience.

![Splashdown Demo](https://via.placeholder.com/800x400?text=Splashdown+Demo)

## ✨ Features

- **Seamless Integration**: Works perfectly with Apollo and Sunshine for Moonlight Game Streaming
- **Zero Interference**: Automatically closes once your game is running
- **Customizable**: Easy to configure with your own splash screens
- **Lightweight**: Minimal resource usage to keep your system running smoothly
- **Fast Startup**: Loads instantly while your game initializes in the background

## 📋 Requirements

- Windows operating system
- Apollo or Sunshine game streaming setup
- Moonlight client

## 🚀 Installation

1. Download the latest release of Splashdown from the [Releases](https://github.com/yourusername/splashdown/releases) page.
2. Extract the ZIP file to a location of your choice.
3. No further installation required – Splashdown is portable!

## 💻 Usage

### Basic Setup

Add Splashdown as a wrapper for your game in Apollo or Sunshine:

```
"C:\Path\To\Splashdown.exe" "C:\Path\To\YourGame.exe" [game parameters]
```

### Configuration Options

Splashdown supports several command-line options:

| Option | Description | Example |
|--------|-------------|---------|
| `--splash-image` | Path to custom splash image | `--splash-image="C:\path\to\image.png"` |
| `--timeout` | Maximum time to wait (seconds) | `--timeout=120` |
| `--fullscreen` | Launch splash in fullscreen mode | `--fullscreen` |
| `--position` | Screen position (center, top, bottom) | `--position=center` |

### Example Configuration

```
"C:\Games\Splashdown\Splashdown.exe" --splash-image="C:\Splashes\MyGame.png" --fullscreen "C:\Games\Epic\MyGame.exe" -epicarg1 -epicarg2
```

## 🔍 How It Works

1. Splashdown launches and immediately displays the splash screen
2. Your game executable is started in the background
3. Splashdown monitors for the game window to appear
4. When the game window is detected, Splashdown automatically closes

## 🛠️ Troubleshooting

**Splash screen won't close:**
- Increase the timeout value with `--timeout=X` where X is seconds
- Verify the game process is actually starting

**Game loads but splash remains:**
- Some games use launcher processes which can confuse detection
- Use `--process-name="ActualGame.exe"` to specify the exact process to wait for

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📊 Feedback & Contributions

Feedback and contributions are welcome! Please feel free to submit a Pull Request or open an Issue on GitHub.

---

*Splashdown is not affiliated with Moonlight, Apollo, or Sunshine projects.*