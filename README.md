# Clipboard Manager

A simple clipboard manager for GNOME desktops.

## Features

*   Access your clipboard history.
*   Simple and lightweight.
*   Integrates with the GNOME desktop environment.

## Prerequisites

Before you begin, ensure you have the following installed:

*   Rust and Cargo
*   CMake
*   `pkg-config`
*   A GNOME desktop environment
*   `gpaste-2`

The build script will attempt to install `gpaste-2` if it is not found, but it is recommended to install it manually using your distribution's package manager.

For Debian/Ubuntu-based systems:
```bash
sudo apt-get install -y libgpaste-2-0 libgpaste-dev gpaste gnome-shell-extensions-gpaste
```

For Fedora/RHEL-based systems:
```bash
sudo dnf install -y gpaste gpaste-devel gnome-shell-extension-gpaste
```

For Arch-based systems:
```bash
sudo pacman -S --noconfirm gpaste gnome-shell-extension-gpaste
```

## Installation

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd clipboard_manager
    ```

2.  **Create a build directory:**
    ```bash
    mkdir build && cd build
    ```

3.  **Configure the build with CMake:**
    ```bash
    cmake ..
    ```

4.  **Compile the project:**
    ```bash
    make
    ```

5.  **Install the application:**
    ```bash
    sudo make install
    ```

## Usage

After installation, you can set up the keybinding to launch the clipboard manager.

**Set up the keybinding (Ctrl+Alt+V):**
```bash
make setup_keybinding
```

You can now use `Ctrl+Alt+V` to open the clipboard manager.

To remove the keybinding, run:
```bash
make remove_keybinding
```

## Uninstallation

To uninstall the application, run the following command from the `build` directory:

```bash
sudo make uninstall
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
