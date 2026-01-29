# MobileHero Launcher 🎸

The PC companion app for **MobileHero**, turning your smartphone into a virtual guitar controller for Clone Hero and other rhythm games.

![MobileHero Launcher](https://via.placeholder.com/800x400?text=MobileHero+Launcher+Preview)

## Features
- **Easy Connection**: Generates a QR code for instant pairing with the MobileHero app.
- **Low Latency**: Uses high-performance WebSockets for minimal input delay.
- **Secure**: PIN-based authentication to prevent unauthorized connections.
- **Configurable**: Custom keybindings and server settings.

## Prerequisites
- **Node.js** (v18 or newer)
- **Rust** (for building the backend)
- **WebView2** (Pre-installed on most modern Windows systems)

## installation

### From Source
1.  Clone this repository:
    ```bash
    git clone https://github.com/nothing2me/MobileHeroLauncher.git
    cd MobileHeroLauncher
    ```

2.  Install dependencies:
    ```bash
    npm install
    # or
    yarn install
    ```

3.  Run the application in development mode:
    ```bash
    npm run tauri dev
    ```

## Building for Production
To create a standalone `.exe` installer:

```bash
npm run tauri build
```
The output will be found in `src-tauri/target/release/bundle/nsis/`.

## Usage
1.  Open **MobileHero Launcher** on your PC.
2.  Click **Start Server**.
3.  Open the **MobileHero App** on your phone.
4.  Scan the **QR Code** displayed on the launcher.
5.  Start rocking! 🤘

## Troubleshooting
- **Firewall**: Ensure "MobileHero Launcher" is allowed through your Windows Firewall.
- **Network**: Both devices must be on the **same Wi-Fi network**.

## License
[MIT](LICENSE)
