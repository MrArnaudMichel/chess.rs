# chess-rs

A simple chess game written in Rust with a GTK4 graphical interface. It renders an interactive 8×8 board, lets you select pieces, highlights legal moves, and performs basic piece movement with turn handling.

This project is a learning-oriented GTK + Rust application. It demonstrates:
- GTK4 UI with a responsive grid-based chessboard
- Model–view separation (board/pieces vs. UI)
- Click-to-select and click-to-move interactions
- Basic move validation logic per piece and turn alternation


## Screenshots
If you run the app, you will see an 8×8 board with pieces drawn from the assets/images directory. Light and dark squares are styled via CSS.


## Requirements
You need a recent Rust toolchain and GTK4 installed on your system.

- Rust (via rustup): https://rustup.rs/
- pkg-config (used by gtk-rs to find libraries)
- GTK 4 development libraries

Platform-specific hints:

- Ubuntu/Debian
  - sudo apt update
  - sudo apt install -y build-essential pkg-config libgtk-4-dev

- Fedora
  - sudo dnf install -y gcc pkgconf-pkg-config gtk4-devel

- Arch Linux
  - sudo pacman -S base-devel pkgconf gtk4

- macOS (Homebrew)
  - brew install gtk4 pkg-config
  - Ensure Homebrew’s pkg-config is used (check pkg-config --cflags gtk4)

- Windows (MSYS2, recommended for gtk-rs)
  - Install MSYS2: https://www.msys2.org/
  - In the MSYS2 UCRT64 shell:
    - pacman -Syu
    - pacman -S --needed mingw-w64-ucrt-x86_64-toolchain mingw-w64-ucrt-x86_64-pkgconf mingw-w64-ucrt-x86_64-gtk4
  - Use the UCRT64 environment when building. Ensure that pkg-config resolves gtk4 (pkgconf --cflags gtk4).

If you encounter errors like “Could not find system library gtk4” or “pkg-config not found”, install the packages above for your OS and try again.


## Build and Run
From the project root:

- Debug run: cargo run
- Release build: cargo build --release

The application window should open with the chessboard.


## How to Play
- Click a piece to select it. Valid target squares are highlighted.
- Click a highlighted square to move the selected piece.
- Turns alternate automatically between White and Black.
- You cannot move the opponent’s pieces.

Note: The project focuses on basic movement and UI demonstration. Advanced rules (checkmate/stalemate detection, castling, en passant, promotion dialogs, timers, PGN, etc.) may be incomplete or not implemented yet.


## Project Structure
- src/main.rs — Application entry point
- src/app.rs — GTK application setup and window creation
- src/ui/components/chessboard.rs — Chessboard UI (grid, buttons, images, highlighting, CSS)
- src/model — Core chess model (board, pieces, positions, moves)
- src/style.css — Board colors, selection, and move highlighting
- assets/images — Piece images used by the UI


## Configuration Notes
- Application ID: fr.arnaudmichel.chess-rs (used by GTK application initialization)
- CSS is embedded at runtime from src/style.css
- Piece images are loaded from the filesystem (assets/images) based on the piece type and color


## Troubleshooting
- error: failed to run custom build command for `gtk4-sys` or “Could not find system library gtk4”
  - Install GTK4 dev packages and pkg-config as described above.
- pkg-config not found
  - Install pkg-config (pkgconf) for your OS and ensure it’s in PATH.
- On macOS, if headers aren’t found
  - Ensure `brew --prefix`/opt paths are visible to pkg-config.
- On Windows, if linking fails
  - Ensure you’re in the MSYS2 UCRT64 environment and installed the UCRT64 variants of toolchain/pkgconf/gtk4.


## Contributing
Issues and pull requests are welcome. For larger changes, please open an issue first to discuss what you’d like to change.


## License
[MIT license](LICENSE)
