<p align="center">
  <img src="assets/README/setupercent_banner.png" width="720" alt="setupercent — battery for wireless devices in system tray">
</p>

<p align="center">
  <a href="https://github.com/jocray/setupercent/releases/latest"><img src="https://img.shields.io/github/v/release/jocray/setupercent?color=466ece&label=latest" alt="Latest release"></a>
  <img src="https://img.shields.io/badge/platform-Windows-466ece" alt="Windows">
</p>

<p align="center">
  A Windows system-tray utility that keeps the battery level of your<br>
  wireless mice, headphones, and keyboards right where you need it.
</p>

---

## Why it exists

Wireless mice die at the worst possible moment. Headphones run out mid-call. Keyboards disconnect while you're typing. Windows only shows battery levels for these devices in Bluetooth settings — and even then, not always and not for everything.

**setupercent** fixes that: it puts the battery percentage right in the system tray and keeps it up to date in the background. One glance at the taskbar and you know when it's time to charge.

## What it looks like

<p align="center">
  <img src="assets/README/docs/preview_main.png" width="280" alt="Main window">
</p>

<p align="center"><em>A device is selected — its battery shows up in the tray</em></p>

<p align="center">
  <img src="assets/README/docs/preview_multi.png" width="280" alt="Multiple devices">
</p>

<p align="center"><em>Track several devices at once and switch between them</em></p>

## What it does

- **Battery in the tray** — the number lives right on the icon, always visible, no extra clicks.
- **Works with any wireless device** that reports battery to the system: mice, headphones, keyboards, gamepads.
- **Multiple devices** — pick the ones you care about from a list and switch between them.
- **Configurable interval** — from 1 to 60 minutes, so the system isn't polled needlessly.
- **Runs quietly** — the window hides to the tray when closed, nothing floats on top of your other windows.
- **English and Russian** — switch languages with one click from the title bar.
- **Lightweight** — about 2 MB, no frameworks, no WebView, barely uses any memory.

## How to use

1. Launch `setupercent.exe` — the window opens right away, the tray icon appears.
2. Tick the devices you want to keep an eye on.
3. Set a refresh interval — 5 minutes, for example.
4. Close the window. It tucks into the tray, and the battery percentage keeps updating in the background.

To bring the window back, click the tray icon. To quit, use the tray icon's context menu.

## Installation

Grab the latest version from the [Releases](https://github.com/jocray/setupercent/releases) page and run it — no installation needed. Settings are stored next to the `.exe` in `setupercent.json`.

## Requirements

- Windows 10 or 11 (x64)
- A device that reports its battery level through the standard Windows interface (most modern Bluetooth mice, headphones, and keyboards)

## Source code

The project is open — source, build instructions, and the full version history live in the [GitHub repository](https://github.com/jocray/setupercent). Found a bug or have an idea? Open an issue.