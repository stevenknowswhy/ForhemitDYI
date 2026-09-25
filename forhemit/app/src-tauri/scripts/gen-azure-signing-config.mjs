#!/usr/bin/env node
// Generates the Tauri config overlay that routes Windows signing through
// Azure Artifact Signing: it merges the static release overlay
// (tauri.release.conf.json) with a bundle.windows.signCommand that invokes
// signtool with Microsoft's signing dlib (Azure.CodeSigning.Dlib.dll).
//
// Written at release time by .github/workflows/release.yml (the "Prepare
// Azure Artifact Signing" step) — never committed; tauri.azure-signing.conf.json
// is gitignored. The dlib's metadata.json (which carries the signing account,
// certificate profile, and region endpoint) is generated separately by the
// workflow step; the client secret only ever travels as env vars.
//
// Inputs (env, set by the workflow step):
//   TAURI_AZURE_SIGNTOOL     absolute path to signtool.exe (x64)
//   TAURI_AZURE_DLIB         absolute path to Azure.CodeSigning.Dlib.dll
//   TAURI_AZURE_METADATA     absolute path to the dlib metadata.json
//   TAURI_AZURE_DESCRIPTION  optional /d description (default: Forhemit).
//                            Shows as the installer name in the MSI UAC prompt.

import { readFileSync, writeFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const here = path.dirname(fileURLToPath(import.meta.url))
const srcTauri = path.resolve(here, '..')
const releaseOverlayPath = path.join(srcTauri, 'tauri.release.conf.json')
const outputPath = path.join(srcTauri, 'tauri.azure-signing.conf.json')

const required = {
  TAURI_AZURE_SIGNTOOL: 'absolute path to signtool.exe',
  TAURI_AZURE_DLIB: 'absolute path to Azure.CodeSigning.Dlib.dll',
  TAURI_AZURE_METADATA: 'absolute path to the dlib metadata.json',
}
for (const [name, what] of Object.entries(required)) {
  if (!process.env[name]) {
    console.error(`gen-azure-signing-config: ${name} must be set (${what})`)
    process.exit(1)
  }
}

// Start from the real release overlay so the two can never drift.
const releaseOverlay = JSON.parse(readFileSync(releaseOverlayPath, 'utf8'))
const description = process.env.TAURI_AZURE_DESCRIPTION || 'Forhemit'

const overlay = {
  ...releaseOverlay,
  bundle: {
    ...releaseOverlay.bundle,
    windows: {
      ...releaseOverlay.bundle?.windows,
      // Tauri invokes this instead of its built-in signtool/thumbprint path,
      // once per file it would sign (the NSIS setup.exe and the MSI). `%1`
      // must stay a standalone array element: the bundler replaces an arg
      // that is exactly %1 with the path of the file being signed, so paths
      // with spaces are safe and nothing is reparsed by a shell. Timestamping
      // is mandatory — Artifact Signing certificates live for three days.
      signCommand: {
        cmd: process.env.TAURI_AZURE_SIGNTOOL,
        args: [
          'sign',
          '/v',
          '/fd', 'SHA256',
          '/d', description,
          '/tr', 'http://timestamp.acs.microsoft.com',
          '/td', 'SHA256',
          '/dlib', process.env.TAURI_AZURE_DLIB,
          '/dmdf', process.env.TAURI_AZURE_METADATA,
          '%1',
        ],
      },
    },
  },
}

writeFileSync(outputPath, JSON.stringify(overlay, null, 2) + '\n')
console.log(`gen-azure-signing-config: wrote ${outputPath}`)
