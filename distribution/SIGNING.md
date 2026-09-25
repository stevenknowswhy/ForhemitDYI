# Signing, notarization, and the auto-updater — operator runbook

This is the release-engineering runbook for making Forhemit **downloadable**.
It covers the three trust layers a desktop app needs, what each CI secret
activates, and how the updater refuses what it must refuse.

Three distinct things get "signed" in this pipeline. They are often confused:

| Layer | What it protects | Mechanism | Status |
| --- | --- | --- | --- |
| **OS installer trust** | "Windows/macOS trusts the installer binary" | Windows: Authenticode (signtool) via `.pfx` or Azure Artifact Signing. macOS: Developer ID codesign + notarization | Optional in v1 — owner decision made: Apple Developer Program + Azure Artifact Signing, pending enrollment (checklist below) |
| **Updater artifact trust** | "The update the app installs is the one we built" | Tauri minisign signature over each updater artifact, verified against the public key embedded in the app. **Cannot be disabled** in the updater plugin | Active — public key committed; private key is a CI secret |
| **Distribution channel** | "The download came from us" | HTTPS GitHub Releases | Active |

## The updater public key

`forhemit/app/src-tauri/tauri.conf.json` embeds the minisign **public** key
under `plugins.updater.pubkey`. Every build carries it; the updater refuses to
install anything not signed by the matching private key.

> **⚠️ The updater keypair was rotated on 2026-09-25** (owner-delegated
> generation): the committed `plugins.updater.pubkey` is the current key,
> replacing the original development/test keypair. The **private** key goes
> in the repo secrets `TAURI_SIGNING_PRIVATE_KEY` and
> `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`; until they are set, a recoverable
> copy of the private key lives in the owner's Forhemit project file library
> (`forhemit-updater-signing.key` + `forhemit-updater-signing.key.password`)
> pending storage in the owner's password manager. Once stored there, the
> key lives only in CI secrets / the password manager — never in the repo.
> Installs from tag `v0.0.1-test` embed the retired dev key and cannot
> receive updates — testers reinstall once (see "Key loss" below).
>
> **Future rotations** follow the same procedure:
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
| `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ACCOUNT`, `AZURE_SIGNING_CERT_PROFILE`, `AZURE_SIGNING_ENDPOINT` | Windows Authenticode signing via **Azure Artifact Signing** (formerly Trusted Signing): the workflow runs signtool with Microsoft's signing dlib during `tauri build` and signs the NSIS and MSI installers — no `.pfx`; the certificate lives in the cloud account | Installer unsigned; SmartScreen warning applies |

macOS keychain import and Windows pfx import only run when the
corresponding secret is non-empty — the workflow gates each step on the
mapped environment variable, so an empty secret simply means "skip signing".

**The two Windows paths are mutually exclusive.** Setting both the `.pfx`
secrets (`WINDOWS_CERTIFICATE`) and the Azure Artifact Signing secrets fails
the release at the signing-config gate — the workflow refuses to guess which
signer to use. A **partial** Azure set (some of the six secrets, not all)
also fails loudly: a half-configured cloud signer must not silently ship
unsigned installers. With no Windows signing secrets at all, the installer
ships unsigned with the standard tester labeling (see "Unsigned tester
builds").

## Activating signing (owner checklist)

Both paths were approved as the v1 target: Apple Developer Program for macOS,
Azure Artifact Signing for Windows (Microsoft's cloud signing service,
~$9.99/month Basic tier at the time of writing). Neither can be exercised by
CI until the enrollment steps below are done and the secrets exist; until
then every release is the clearly labeled unsigned tester build.

### macOS — Apple Developer Program

1. Enroll in the Apple Developer Program (organization or individual) and
   wait for activation.
2. Create a **Developer ID Application** certificate (Xcode or
   developer.apple.com → Certificates), download it, and export a `.p12`
   with an export password.
3. Repo secrets:
   - `APPLE_CERTIFICATE` — the `.p12`, base64-encoded (`base64 -i cert.p12`)
   - `APPLE_CERTIFICATE_PASSWORD` — the `.p12` export password
   - `APPLE_SIGNING_IDENTITY` — the certificate's common name, e.g.
     `Developer ID Application: Forhemit (<TEAMID>)`
4. Notarization (tauri uses notarytool): create an **app-specific password**
   at appleid.apple.com for the Apple ID holding the membership, then:
   - `APPLE_ID` — that Apple ID's email
   - `APPLE_PASSWORD` — the app-specific password (not the account password)
   - `APPLE_TEAM_ID` — the 10-character team ID
5. First signed release: the updater keypair was rotated on 2026-09-25
   (warning above), so production updater signatures already start with the
   production key.

### Windows — Azure Artifact Signing (formerly Trusted Signing)

1. In the Azure portal, register the `Microsoft.CodeSigning` resource
   provider, then create an **Artifact Signing account** in a region that
   supports the service and note that region's **endpoint URI** (e.g.
   `https://eus.codesigning.azure.net` for East US — Microsoft's docs have
   the full region table).
2. Complete **identity validation** for the organization (Azure portal only;
   can take 1–20 business days — start early) and create a **certificate
   profile** (Public Trust). Note the account name and profile name.
3. Create an Entra ID **app registration** with a client secret and grant
   the app the **Artifact Signing Certificate Profile Signer** role on the
   profile.
4. Repo secrets — **all six are required**; the workflow refuses a partial
   set:

   | Secret | Value |
   | --- | --- |
   | `AZURE_TENANT_ID` | Entra tenant (directory) ID |
   | `AZURE_CLIENT_ID` | App registration (client) ID |
   | `AZURE_CLIENT_SECRET` | The client secret's **value** |
   | `AZURE_SIGNING_ACCOUNT` | Artifact Signing account name |
   | `AZURE_SIGNING_CERT_PROFILE` | Certificate profile name |
   | `AZURE_SIGNING_ENDPOINT` | Region endpoint URI (step 1) |

5. Tag a test release and confirm the run reports `Windows signing mode:
   azure` and signs the NSIS and MSI installers (each artifact shows a
   signtool "Successfully signed" line in the build log).

There is nothing to import on the runner — the certificate never leaves the
Azure account. The workflow downloads Microsoft's signing dlib, points
signtool at it through the generated `tauri.azure-signing.conf.json` overlay
(`bundle.windows.signCommand`), and the dlib authenticates with the
`AZURE_TENANT_ID` / `AZURE_CLIENT_ID` / `AZURE_CLIENT_SECRET` env vars. The
`.pfx` and Azure paths must never be configured together (see above).

## What the release workflow builds

Tag `v*` push (or manual dispatch) → one job per target:

| Target | Runner | Bundles | Updater artifact |
| --- | --- | --- | --- |
| linux-x86_64 | ubuntu | `appimage`, `deb` | the `.AppImage` itself |
| macos-aarch64 | macos | `app`, `dmg` | `<app>.app.tar.gz` (+ `.sig`) |
| macos-x86_64 | macos | `app`, `dmg` | `<app>.app.tar.gz` (+ `.sig`) |
| windows-x86_64 | windows | `nsis`, `msi` | `<app>-setup.exe` (+ `.sig`) |

When updater artifacts are signed, tauri-action (`uploadUpdaterJson`)
assembles `latest.json` from each target's signed artifacts and attaches it
to the release — which is drafted until **every** target succeeds, so the
visible manifest is always complete. The app checks:

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

- creates the release as a **prerelease**, named "… — unsigned tester build",
- flags the release notes: builds are not notarized / not Authenticode-signed
  and the auto-updater is disabled (artifacts carry no updater signature).

Tester install steps (including the Gatekeeper / SmartScreen workarounds
unsigned builds require) are in [INSTALL.md](./INSTALL.md).
