# rustymon
A Pokémon-style game written in Rust for our computer science capstone project.

For cross compilation on Linux to Windows:
1. Download SDL2 mingw64 devel, SDL2_image mingw64 devel, and SDL2_ttf mingw64 devel from respective repositories
2. Unzip
3. `cp -r SDL2-<ver>/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib && cp -r SDL2_image-<ver>/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib`
4. Copy SDL2.dll and SDL2_image.dll from the respective `mingw32/bin` directories into the rustymon folder
5. Download SDL2 ttf win32-x64 zip and extract dll to Rustymon folder.

![mainmenu](https://user-images.githubusercontent.com/19539925/195476812-b88f644f-2291-4b71-895d-8e0c2bfbb9d9.PNG)
![overworld_tiles](https://user-images.githubusercontent.com/19539925/195476817-0678bce0-2770-40df-bb3f-72e0ac89d864.PNG)
