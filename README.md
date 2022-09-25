# rustymon
A Pokémon-style game written in Rust for our computer science capstone project.

For cross compilation on Linux to Windows:
1. Download SDL2 Windows devel from the repository
2. Unzip
3. `cp -r SDL2-<ver>/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib`
