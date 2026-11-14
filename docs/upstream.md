# Trama upstream

| Field | Value |
|---|---|
| Upstream | `storytold/figcraft` |
| Last observed | `bad07e4433240cad046589129c084bd2fc36f097` |
| Last evaluated | `bad07e4433240cad046589129c084bd2fc36f097` |
| Last integrated | `bad07e4433240cad046589129c084bd2fc36f097`, merged by the pull request that named the app |
| Skipped | the default online-collaboration server (`collab.craftapps.dev`, an upstream service), its terms page and the upstream brand kit (`docs/brand/`) |
| Adapted | names, app id `dev.danielmiguelt.Trama`, the in-app updater (`nodalia-update`), the app icon, the theme (NDL with Trama's accent), the title bar on Linux (NDL's window controls) |
| Conflicting | `README.md`, `AGENTS.md` and the workflows: this repository's are kept |

## How the sync works

Upstream's code arrives renamed: `scripts/rebrand.py` applies `.github/rebrand.toml` (FigCraft
becomes Trama, internal names included) to upstream's tree, and the weekly sync
(`.github/workflows/upstream-sync.yml`) records the result on the `upstream-trama` branch. Each
commit there has the previous mirror commit and the upstream commit it renames as parents, and
names that commit in an `Upstream-Commit:` trailer. The sync merges the newest mirror commit into a
branch and opens a pull request; files listed in `.github/upstream-keep` keep this repository's
version.

The mirror starts at the renamed `bad07e44`, the first commit on this repository's `main` after
upstream's history. `python3 scripts/rebrand.py check` fails when an old name comes back; a line
that must keep one says `rebrand: keep`.

## Names that stay on purpose

Files and plugins made before the rename keep working, so a few spans keep the old name
(`.github/rebrand.toml` lists them): the native file's signature `"format": "figcraft"`, the
`.figcraft` extension (opened and saved like `.trama`), the plugin ABI (module `figcraft`,
`figcraft_run`), the old plugin manifest name `figcraft-plugin.json`, the `figcraft://` invite
links and the data Trama stores in exported `.fig` files. Upstream's credit (the ArtCraft Team and
the FigCraft contributors) stays in `NOTICE`, `LICENSE-MIT` and About.
