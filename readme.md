# 🚀☄️🌊 Splashdown

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/philipgyllhamn/splashdown?style=social)](https://github.com/yourusername/splashdown)

> **Enhance your game streaming experience with seamless splashscreen for a professional Xcloud look**

Splashdown is a lightweight utility designed specifically for Apollo and Sunshine game streaming environments. It displays an splash screen while your game loads in the background, creating a seamless and professional streaming experience. Similar to Xbox Cloud splash screen.

## ✨ Features

- **Seamless Integration**: Works perfectly with Apollo and Sunshine for Moonlight Game Streaming
- **Zero Interference**: Automatically closes once your game is running
- **Lightweight**: Minimal resource usage to keep your system running smoothly
- **Fast Startup**: Loads instantly while your game initializes in the background

## 📋 Requirements

- Windows operating system

## 🚀 Installation

1. Download the latest release of Splashdown from the [Releases](https://github.com/philipgyllhamn/splashdown/releases) page.
2. Extract the ZIP file to a location of your choice or just download the .exe directly.
3. No further installation required – Splashdown is portable!

## 💻 Usage

### Basic Setup

Add Splashdown as a wrapper for your game in Apollo or Sunshine:

add the below parameters to "Detached Command" inside your Sunshine/Apollo dashboard for your specific application.

```
C:\Path\To\splashdown.exe "C:\Path\To\YourGame.exe"
```

### *You need to check the "Run as Admin" box for it to work aswell*

## 🔍 How It Works

1. Splashdown launches and immediately displays the splash screen
2. Your game executable is started in the background
3. Splashdown monitors for the game window to appear
4. When the game window is detected, Splashdown automatically closes

<!-- ## 🛠️ Troubleshooting

**Splash screen won't close:**
- Increase the timeout value with `--timeout=X` where X is seconds
- Verify the game process is actually starting

**Game loads but splash remains:**
- Some games use launcher processes which can confuse detection
- Use `--process-name="ActualGame.exe"` to specify the exact process to wait for -->

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📊 Feedback & Contributions

Feedback and contributions are welcome! Please feel free to submit a Pull Request or open an Issue on GitHub.

---

*Splashdown is not affiliated with Moonlight, Apollo, or Sunshine projects.*
