# Recipes

This directory holds two BlueBuild recipes, built in parallel by the same
GitHub Actions workflow (see `.github/workflows/build.yml`):

| Recipe | Image | Base image | Purpose |
| --- | --- | --- | --- |
| `recipe.yml` | `ghcr.io/dverdonschot/fedora-cosmic-atomic-ewt` | `quay.io/fedora-ostree-desktops/cosmic-atomic` | Daily driver, COSMIC desktop |
| `recipe-gnome.yml` | `ghcr.io/dverdonschot/fedora-gnome-atomic-ewt` | `quay.io/fedora-ostree-desktops/silverblue` | Fallback / sanity-check image |

## Why two recipes in one repo

The GNOME image exists so that, when something on the COSMIC image misbehaves,
you can rebase to GNOME and isolate whether the cause is COSMIC or your
personal layer (rootless Docker, AMD kargs, custom tooling, …). Both recipes
share the same `files/`, helper scripts, dnf package list, systemd tweaks,
flatpaks, and Fedora major version — the only deliberate difference is the
base image.

## Keeping them in sync

The two recipe files are 95% identical by design. If you change one, ask
yourself: **does this change apply to both images?**

- **Helper scripts, AMD kargs, rootless Docker, forgejo-runner, the dnf
  package list, the flatpak list, the systemd module** — change both.
- **Anything desktop-specific** (gnome-extensions, cosmic theming,
  gnome-tweaks, …) — change only the recipe it belongs to. If you find
  yourself adding desktop-specific bits to a recipe, prefer a
  `files/<recipe>/` overlay directory rather than weaving the conditional
  into the shared sections.

A quick way to spot drift between the two:

```bash
diff -u recipes/recipe.yml recipes/recipe-gnome.yml
```

The current expected diff is exactly four lines: the `name:` field, the
`description:` field, the `base-image:` field, and the comment block above
the base-image line.

## Image name vs recipe name

BlueBuild derives the published image name from each recipe's `name:`
field. The two recipes produce two distinct GHCR packages even though they
live in one repo. If you ever want to also publish the GNOME image under a
third name (e.g. an LTS pin), add a new recipe file and append it to the
matrix in `.github/workflows/build.yml`.
