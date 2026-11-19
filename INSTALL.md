# Install Memory Pairs

1. Open **Store** from Primitive Launcher.
2. Search for **Memory Pairs** and tap **Get**.
3. Open **Memory Pairs** from the Launcher home screen.

Development builds can be installed by USB on an ARMHF Kindle running firmware 5.16.3 or newer. Check the release ZIP
against its SHA-256 file, then extract it. Copy the `memorypairs/` payload folder to the Kindle drive root and
`documents/MemoryPairs.sh` into the existing `documents/` folder. Eject the Kindle before opening the game.

The game uses Launcher’s shared fonts and display settings. It saves progress in `memorypairs/memorypairs.json` on the Kindle.
