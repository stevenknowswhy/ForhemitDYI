# Installing Forhemit — per-OS steps

Releases live at
`https://github.com/stevenknowswhy/ForhemitDYI/releases`
(one release per `v*` tag; unsigned tester builds are `-unsigned` prereleases).

v1 is local-only: the app never uploads your information. The only outbound
network call is an **explicit** "Check for updates" request to GitHub
Releases; the updater refuses unsigned or stale update manifests before
downloading anything.

## macOS

1. Download `Forhemit_<version>_aarch64.dmg` (Apple Silicon) or
   `..._x86_64.dmg` (Intel) from the release page.
2. Open the `.dmg` and drag **Forhemit** into `Applications`.
3. First launch, unsigned build: right-click the app → **Open** → **Open**
   (Gatekeeper warning — the build is **not yet notarized**; this prompt is
   expected). Signed + notarized builds (once certs are configured) open
   without this step.

On Apple Silicon, unsigned builds are ad-hoc signed so they launch at all;
macOS still requires the right-click allowance above.

## Windows

1. Download `Forhemit_<version>_x64-setup.exe` (NSIS installer) or
   `Forhemit_<version>_x64_en-US.msi` from the release page.
2. Run the installer. Unsigned build: SmartScreen shows
   **"Windows protected your PC"** → *More info* → **Run anyway**
   (the build is not Authenticode-signed yet).
3. Launch **Forhemit** from the Start menu.

## Linux

Debian/Ubuntu (`.deb`):

```sh
sudo apt install ./Forhemit_<version>_amd64.deb
# launches from the desktop environment; binary: /usr/bin/forhemit
```

Any distribution (AppImage):

```sh
chmod +x Forhemit_<version>_amd64.AppImage
./Forhemit_<version>_amd64.AppImage
```

Requires WebKitGTK 4.1 (`libwebkit2gtk-4.1-0`) at runtime.

## Updating

In the app, use **Check for updates** (footer). The updater only ever offers
a strictly newer, correctly signed release; it refuses unsigned or stale
manifests and says so. Installing applies the update; you restart when
convenient.

## Verifying an install (smoke test)

- macOS: `codesign -dv /Applications/Forhemit.app` (ad-hoc builds show
  `Signature=adhoc`).
- Windows: right-click the installer → *Properties* → *Digital Signatures*
  (absent on unsigned builds).
- Linux: `dpkg -L forhemit` lists installed files; the AppImage runs
  `--appimage-extract` to prove the image is intact.
