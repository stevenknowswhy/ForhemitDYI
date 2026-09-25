# Signing, notarization, and the auto-updater — operator runbook

This is the release-engineering runbook for making Forhemit **downloadable**.
It covers the three trust layers a desktop app needs, what each CI secret
activates, and how the updater refuses what it must refuse.

Three distinct things get "signed" in this pipeline. They are often confused:

| Layer | What it protects | Mechanism | Status |
| --- | --- | --- | --- |
| **OS installer trust** | "Windows/macOS trusts the installer binary" | Windows: Authenticode (signtool). macOS: Developer ID codesign + notarization | Optional in v1 — pending certs (open owner decision) |
| **Updater artifact trust** | "The update the app installs is the one we built" | Tauri minisign signature over each updater artifact, verified against the public key embedded in the app. **Cannot be disabled** in the updater plugin | Active — public key committed; private key is a CI secret |
| **Distribution channel** | "The download came from us" | HTTPS GitHub Releases | Active |

## The updater public key

`forhemit/app/src-tauri/tauri.conf.json` embeds the minisign **public** key
under `plugins.updater.pubkey`. Every build carries it; the updater refuses to
install anything not signed by the matching private key.

> **⚠️ The currently committed key is a development/test keypair.**
> It was generated to wire and test the pipeline before signing credentials
> exist (the open owner decision: Apple Developer ID + Windows certificate —
> see the pinned spec, "Open questions"). **Rotate before any public
> distribution:**
>
> 1. `cd forhemit/app && npm run tauri signer generate -- -w ~/.tauri/forhemit.key`
>    (use a password; keep it with the key)
> 2. Put the **private key file's content or path** in the repo secret
>    `TAURI_SIGNING_PRIVATE_KEY`, and the password in
>    `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
> 3. Replace `plugins.updater.pubkey` in `tauri.conf.json` with the new
>    **public** key's base64 line (the `.pub` file body).
> 4. Publish one final release signed with the old key for users on old
>    builds, or accept that they reinstall (see "Key loss" below).

**Key loss:** if the private key (or its password) is lost, installed apps
can no longer accept updates — they must reinstall from a fresh download.
This is why the private key lives only in CI secrets / a password manager,
never in the repo.

## CI secrets — what activates what

The release workflow (`.github/workflows/release.yml`) checks secret
presence per job. **Dropping a secret in activates the matching behavior
with no code changes**; absent secrets produce clearly labeled unsigned
tester builds.

| Secret(s) | Activates | When absent |
| --- | --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` (+ `_PASSWORD`) | Updater artifacts built + minisign-signed; `latest.json` published to the release | Build skips updater artifacts; artifacts labeled unsigned; no `latest.json` |
| `APPLE_CERTIFICATE` (base64 `.p12`), `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY` | macOS Developer ID signing | Ad-hoc signing (`signingIdentity: "-"`) — launchable, Gatekeeper-untrusted |
| `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` (or `APPLE_API_ISSUER`, `APPLE_API_KEY_PATH`) | macOS notarization + stapling during `tauri build` | Build labeled **not yet notarized** |
| `WINDOWS_CERTIFICATE` (base64 `.pfx`), `WINDOWS_CERTIFICATE_PASSWORD` | Windows Authenticode signing (CI imports the cert, computes the thumbprint, and passes it via a generated config overlay) | Installer unsigned; SmartScreen warning applies |

macOS keychain import and Windows pfx import only run when the
corresponding secret is non-empty — the workflow gates each step on the
mapped environment variable, so an empty secret simply means "skip signing".

## What the release workflow builds

Tag `v*` push (or manual dispatch) → one job per target:

| Target | Runner | Bundles | Updater artifact |
| --- | --- | --- | --- |
| linux-x86_64 | ubuntu | `appimage`, `deb` | the `.AppImage` itself |
| macos-aarch64 | macos | `app`, `dmg` | `<app>.app.tar.gz` (+ `.sig`) |
| macos-x86_64 | macos | `app`, `dmg` | `<app>.app.tar.gz` (+ `.sig`) |
| windows-x86_64 | windows | `nsis`, `msi` | `<app>-setup.exe` (+ `.sig`) |

When updater artifacts are signed, a `release` job assembles `latest.json`
(`scripts/build-updater-manifest.py`) and attaches it; the app checks:

```
https://github.com/stevenknowswhy/ForhemitDYI/releases/latest/download/latest.json
```

## Updater refusal rules (what the tests pin)

The updater runs two layers of refusal; both must fail **loudly**, never
silently degrade:

1. **Pre-flight verdict** (`forhemit/app/src-tauri/src/updater_manifest.rs`,
   unit-tested in CI): the manifest must be well-formed JSON, offer a
   **strictly newer** semver than the running app (a stale or downgrade
   manifest is refused), carry a parseable RFC 3339 `pub_date` (a future
   date beyond clock tolerance is refused), have an entry for the running
   platform, and carry a **non-empty, decodable minisign signature** — an
   unsigned entry is refused **before any bytes are downloaded**.
2. **Plugin enforcement** (tauri-plugin-updater, upstream): the downloaded
   artifact's minisign signature is verified against the embedded public
   key; verification **cannot be disabled**, and a refused artifact is never
   installed.

Unsigned tester builds ship **no** `latest.json` — installed apps report
"no update information available" rather than silently installing anything.

## Unsigned tester builds — labeling

When no signing secrets are configured, the workflow:

- appends `-unsigned` to every artifact filename,
- marks the GitHub release as a **prerelease**,
- flags the release notes: builds are not notarized / not Authenticode-signed.

Tester install steps (including the Gatekeeper / SmartScreen workarounds
unsigned builds require) are in [INSTALL.md](./INSTALL.md).
