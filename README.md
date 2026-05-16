# EchoClip

**A lightweight clipboard manager for Linux and Windows**

EchoClip automatically saves your clipboard history — texts, links, and images — with a clean interface and system tray support.

## ✨ Features

- **Real-time clipboard monitoring**
- **Full support** for Text, Links, and Images (PNG, JPEG, BMP)
- **Smart duplicate detection** — no repeated entries
- **System tray integration** with autostart
- **Right-click** any item to copy it back to clipboard
- **Lightweight & fast** – built with Avalonia UI
- **Single-file executable** on Windows (no installer needed)
- **Easy installation** on Linux via script

## 📥 Downloads

Latest release: **[v1.0.0](https://github.com/Spiritecho777/EchoClip/releases/tag/v1.0.0)**

- **Linux**: `EchoClip.tar` (Recommended)
- **Windows**: `EchoClip.exe` (Static single-file)

## 🛠️ Installation

### Linux
```bash
# Download and extract
tar -xvf EchoClip.tar

# Install
cd EchoClip
chmod +x install.sh
sudo ./install.sh
```

The installer automatically configures:

Application files
Desktop icon
Application menu entry
Autostart in system tray

### Windows
Just download and run EchoClip.exe. No installation required.

## 📖 Usage

Launch the app normally to open the history window
It runs in the system tray by default
Right-click any item in the list to copy it back to clipboard

## 🏗️ Build from Source
Bashdotnet restore
dotnet build -c Release
For publishing:

Windows: dotnet publish -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true
Linux: dotnet publish -c Release -r linux-x64 --self-contained true -p:PublishSingleFile=true

## 📸 Screenshots
(À ajouter plus tard)

## License
This project is licensed under the MIT License.
Contributions, bug reports, and feature requests are welcome! Feel free to open an issue or submit a pull request.
License
This project is licensed under the MIT License
