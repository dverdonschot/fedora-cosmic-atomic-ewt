# Forgejo Runner on UM790 — Implementation Plan

Author: AI agent (ewt)
Date: 2026-06-29
Status: PLANNING — awaiting sign-off, no implementation yet.

## Context

The Odroid (NixOS) already runs the Forgejo Actions runner
(`dverdonschot/nixos-systems-configuration`, PR #7 merged
`312ad42` on 2026-06-29). Goal: bring the UM790 (Fedora Atomic +
COSMIC, AMD Ryzen + AMD780 iGPU) into the same fleet as a runner
for **large jobs and AMD-accelerated jobs**.

Three decisions locked in by the user:

| Decision | Choice |
|---|---|
| Module home | `modules/forgejo-runner` in `fedora-cosmic-atomic-ewt` (mirrors Odroid's pattern) |
| GPU mode | Full ROCm + GPU scheduling (RDNA3 iGPU via `/dev/dri` + `/dev/kfd`) |
| Auth path | `~/.config/forgejo/runners/um790/` (user-owned; system rootless docker) |

## Constraints (atomic-specific)

1. **Image is `rpm-ostree`-based.** Layering only happens at build
   time inside `recipe.yml`. Per-user services (`systemctl --user`)
   cannot be enabled at image-build time — the `~/.config/systemd/user/`
   tree doesn't exist yet, so any `--user` enable would have to happen
   post-rebase. Same shape as the existing `setup-rootless-docker.sh`.

2. **`/usr/local` is a symlink to `/var/usrlocal`.** Existing
   `recipe.yml` already handles this with single-file
   `source:`/`destination:` entries (commented in
   `setup-rootless-docker.sh`). New Forgejo files must follow the
   same pattern, not bulk-copy.

3. **System `docker.service` is masked.** Per-user Docker only,
   driven by `setup-rootless-docker.sh` (existing, post-rebase).
   The runner will live alongside rootless Docker and reuse its
   `/run/user/<UID>/docker.sock`.

4. **AMD780 = Navi 24 (RDNA 3).** Mesa userspace already in the
   image (`mesa-libOpenCL-devel`). ROCm runtime NOT installed
   yet — adding `rocm-runtime`, `rocm-clinfo`, `rocm-opencl` and
   gating `/dev/kfd` access requires careful container config
   (KFD requires group membership of `render`/`video`).

## Out of scope for this plan

- Adding UM790 to **`dverdonschot/nixos-systems-configuration`'s
  `hosts/um790/configuration.nix`**. That's a sister repo and a
  separate task. (The Odroid runner is a NixOS module; the UM790
  runner is a BlueBuild image. They share the **Forgejo instance**
  but not the **deployment recipe**.)
- A Forgejo-side group/label registry update — that's a Forgejo UI
  change, not a host change.
- Any sops/age secret-management — token stays as a plain file
  under `~/.config/forgejo/runners/um790/token` until we add sops.

## Repo changes

### Repo A: `dverdonschot/fedora-cosmic-atomic-ewt` (this branch: `feat/forgejo-runner`)

1. **`modules/forgejo-runner/config.toml.j2`** —
   Runner config template. Carved out so the recipe can drop a
   rendered version at `/etc/forgejo-runner/config.toml` and the
   post-rebase script keeps it editable. Fields:
   - `log.level`, `runner.file`, `runner.capacity`, `runner.timeout`
   - `cache.enabled = false` (v1; consistent with Odroid)
   - `container.docker_host = "unix:///run/user/<UID>/docker.sock"`
     (commented-out + filled in by the setup script; UID-at-runtime)
   - `container.runtime = "runc"`
   - `container.options = "--device /dev/dri --device /dev/kfd"`
     (commented as the deploy-time default; `--privileged=false`)
   - `host.workdir_parent = "/var/tmp/forgejo-runner/work"`
   - Top-level `[[runners]]` array with `name = "um790-runner"`,
     `url`, `token_url = "file:/.../token"`, `labels = [..., "gpu=amdgpu"]`.

2. **`modules/forgejo-runner/setup-forgejo-runner.sh`** —
   Post-rebase one-time setup, modeled on
   `files/rootless-docker/bin/setup-rootless-docker.sh`. Steps:
   1. Refuse root.
   2. Verify docker rootless is up (via `verify-rootless-docker.sh`).
   3. Verify `/dev/dri/renderD128` is present, and that `groups`
      includes `render` and `video` (exit with a clear message if
      not, because Fedora Atomic's group setup is layered).
   4. Materialize per-user token file path
      `~/.config/forgejo/runners/um790/token` (mode 0600).
   5. Render the template into `~/.config/forgejo-runner/config.toml`
      using `id -u` to fill the UID-in-socket-path.
   6. Enable + start `forgejo-runner.service` in the user systemd
      instance (a `[Unit]` snippet installed at image-build time).
   7. Verify: `systemctl --user is-active forgejo-runner.service`,
      `forgejo-runner --config ... verify` if the binary supports it.

3. **`modules/forgejo-runner/forgejo-runner.service`** —
   User-mode systemd unit. **Default `enabled=false`, `started=false`**
   per the user's spec ("install + configure, do not start, do not
   authorize"). When the operator runs
   `setup-forgejo-runner.sh`, the script flips both flags.
   - `WorkingDirectory=%h/.local/share/forgejo-runner`
   - `ExecStart=/usr/bin/forgejo-runner daemon --config %h/.config/forgejo-runner/config.toml`
   - `Environment=DOCKER_HOST=unix:///run/user/%U/run/docker.sock`
     (set by the setup script, not here, to avoid a chicken-and-egg
     with the variable substitution at unit-load time)
   - `Restart=on-failure`, `RestartSec=20s`
   - Hardening subset that survives user-mode (no `ProtectSystem` at
     this level — that lives in the rootless docker side already).

4. **`recipes/recipe.yml`** — add the systemd unit + binary:
   - **dnf install:** `forgejo-runner` (Fedora 44 package present in
     updates-testing; if not, fallback to a vendored binary in
     `modules/forgejo-runner/forgejo-runner-bin/...`).
   - **dnf install for ROCm:** `rocm-runtime`, `rocm-clinfo`,
     `rocm-opencl`, `rocminfo`, plus ensure `kernel-modules-extra`
     is present (`amdgpu` driver shipped in main kernel).
   - **Add to cosign path:** drop the user-mode systemd unit under
     `/usr/lib/systemd/user/forgejo-runner.service` (symlink to
     `~/.config/systemd/user/` doesn't work as image-baked; the
     user unit dir is per-user so we install the unit in
     `/usr/lib/systemd/user/` and `systemctl --user` will pick it up
     on first user login).
   - **systemd module:** **do NOT** `systemctl enable` anything at
     image-build time. Only `systemd.user.enabled` is irrelevant
     here; we install the unit file but flip the user-mode enable in
     the post-rebase script. Per the user's "configure not start"
     requirement.
   - **files entries:** add the rendered config template under
     `/etc/forgejo-runner/config.toml.j2` and the setup script under
     `/usr/local/bin/setup-forgejo-runner.sh`. **Don't** symlink;
     /usr/local works as individual-file copy per existing convention.

5. **YADM bootstrap for this host:** Modify `~/.yadm-bootstrap` to
   also wire the Forgejo-runner post-rebase setup. Since the runner
   setup script is shipped in `/usr/local/bin/`, the bootstrap
   doesn't need to copy files — it just calls the script.

### Repo B: `dverdonschot/cosmic-yadm` (this branch: `feat/forgejo-runner-env`)

The yadm repo is **host-state dotfiles**, not packages. Cosmetic
additions only:

6. **`~/.config/forgejo-runner/.gitignore-template`** — A file
   saying what's "ephemeral in the user's home" (none, currently;
   the config under `~/.config/forgejo-runner/` should NOT be
   yadm-tracked). Shipped so a future session knows the rule.

No structural changes. Yadm is already pulling in the binary + service
unit via the image.

### Repo C (open item): `dverdonschot/nixos-systems-configuration`

Adding UM790 to `hosts/um790/configuration.nix` would mirror the
odroid block. **Not in scope for this plan.** Flagged for follow-up.

## Recipe.yml delta (sketch)

```yaml
modules:
  ...
  - type: dnf
    install:
      packages:
        ...
        # Forgejo runner
        - forgejo-runner
        # ROCm (RDNA3 iGPU on AMD780)
        - rocm-runtime
        - rocm-clinfo
        - rocm-opencl
        - rocminfo
  - type: files
    files:
      - source: modules/forgejo-runner/forgejo-runner.service
        destination: /usr/lib/systemd/user/forgejo-runner.service
      - source: modules/forgejo-runner/setup-forgejo-runner.sh
        destination: /usr/local/bin/setup-forgejo-runner.sh
      - source: modules/forgejo-runner/config.toml.j2
        destination: /etc/forgejo-runner/config.toml.j2
```

A new `systemd-tmpfiles` snippet or `EnvironmentFile=` snippet to wire
the GPU devices through the per-user systemd instance isn't strictly
required (devices are controlled by Linux device group on Fedora Atomic
groups and membership is user-mode controlled). The setup script's
group check covers that.

## Acceptance criteria

Plan-complete:

- [ ] `docs/forgejo-runner-um790-plan.md` written and committed.
- [ ] User sign-off in this conversation ("go on plan").

Implementation-complete:

- [ ] Branch `feat/forgejo-runner` in `dverdonschot/fedora-cosmic-atomic-ewt`
      with all 5 module items above.
- [ ] Image builds cleanly on `forgejo-recipe.yml` bluebuild run; no
      `recipe.yml` parse errors.
- [ ] Rebase workflow: operator runs `setup-forgejo-runner.sh`,
      drops token in `~/.config/forgejo/runners/um790/token`, runs
      `systemctl --user enable --now forgejo-runner.service`.
- [ ] Runner appears in Forgejo UI as `um790-runner` with labels
      `host=um790,self-hosted,linux,x86_64,size=large,gpu=amdgpu`.
- [ ] Smoke test: workflow on the runner can read `/dev/dri/renderD128`
      and `rocminfo` reports the AMD780.

## Risks / open questions

1. **`forgejo-runner` RPM availability.** Fedora 44 may not have
   it packaged yet. If absent, fallback: install a vendored binary
   from upstream release tarball under
   `modules/forgejo-runner/bin/forgejo-runner`. **Need to verify**
   by running `dnf search forgejo-runner` at plan-execution time.

2. **ROCm + rootless Docker + container-selinux.** The kernel
   refuses to expose `/dev/kfd` to a container unless the
   container process is in the right cgroup + has `CAP_SYS_PTRACE`
   or similar. Forgejo-runner spawns containers via the rootless
   docker socket; the spawned container inherits the user's cgroup.
   We may need to tune `dockerd-rootless-setuptool.sh` flags to
   allow KFD access. **Mitigation:** document the command in the
   post-rebase script; flag as test-time issue.

3. **Token file lifecycle.** `setup-forgejo-runner.sh` creates
   the `~/.config/forgejo/runners/um790/` tree. The operator
   places the token manually. The script does not overwrite an
   existing token (idempotent on token-read step), so rotation
   means `rm ~/.config/forgejo/runners/um790/token && re-run`.

4. **Open item mentioned above:** the UM790 is currently absent
   from `nixos-systems-configuration`'s `hosts/um790/`. **This
   isn't a problem for the runner module itself** (the UM790
   runs Fedora Atomic, not NixOS), but it is a problem for the
   "UM790 also has NixOS-style hosts" mental model. Resolved
   outside this plan.

5. **Naming conflict with the **Odroid** module.** The
   `dverdonschot/nixos-systems-configuration` module is also
   called `modules/forgejo-runner.nix`. Different filenames
   (`modules/forgejo-runner/` dir vs `modules/forgejo-runner.nix`
   file), different hosts, different recipe. **Convention:**
   keep the Fedora version's directory layout because:
   (a) it's a blueprint, not a single Nix expression,
   (b) yadm doesn't deal with `.nix` files anyway.
   Document the dual naming in MEMORY.md after plan-complete.
