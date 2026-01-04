# Nova System Monitor 🚀

**Nova** is a modern, high-performance system monitor built with **Tauri**, **Rust**, and **Svelte 5**. It combines the aesthetics of modern web design with the performance of native code.

![Nova System Monitor](https://github.com/user-attachments/assets/placeholder-image)
*(Screenshot placeholder - will be added later)*

## ✨ Key Features

### 🖥️ Modern Dashboard
-   **Aesthetic First**: Built with **Material 3** design principles.
-   **Theming**: Support for **Light**, **Dark**, **Mocha**, and **Cappuccino** themes.
-   **Responsive**: Smooth, hardware-accelerated animations (60fps).

### ⚡ Performance & Hardware
-   **GPU Monitoring**: Detailed stats for **NVIDIA** (NVML), **AMD** (Sysfs), and **Intel** (RC6 Residency).
-   **Disk I/O**: Real-time Read/Write graphs + **SMART Health** status.
-   **Process Management**:
    -   **Tree View**: Visualize parent/child process relationships.
    -   **Detailed Inspection**: View process "Nice" priority, full command line arguments, and execution paths.
-   **Network**: Real-time upload/download history per interface.

### 🛡️ Why Nova?
| Feature | Nova System Monitor | Standard / Gnome Monitor |
| :--- | :--- | :--- |
| **GPU Stats** | ✅ Multi-vendor & Detailed | ❌ Basic / Extension required |
| **Disk Health** | ✅ Integrated SMART Status | ❌ Requires separate app |
| **Tech Stack** | Rust + Svelte (Modern) | C/C++ + GTK |

## 🛠️ Installation

### Prerequisites
-   Linux (tested on Ubuntu/Pop!_OS)
-   `libwebkit2gtk-4.0-dev` (for Tauri)
-   `lm-sensors` (optional, for extra sensors)

### Build from Source
```bash
# 1. Clone Repositori
git clone https://github.com/WRVbit/nova-systemmonitor.git
cd nova-systemmonitor

# 2. Install Dependencies
npm install

# 3. Run Development Mode
npm run tauri dev

# 4. Build Release Bundle (.deb)
npm run tauri build
```

## 🏗️ Technology Stack
-   **Backend**: Rust (Sysinfo, libc, nvml-wrapper, tokio)
-   **Frontend**: SvelteKit (Svelte 5 Runes), TypeScript
-   **Framework**: Tauri v2
-   **Styling**: Vanilla CSS (Variables based theming) + Lucide Icons

## 📝 License
MIT License - Created by **Alfarez** (WRVbit).

