# fedora-cosmic-atomic-ewt &nbsp; [![bluebuild build badge](https://github.com/dverdonschot/fedora-cosmic-atomic-ewt/actions/workflows/build.yml/badge.svg)](https://github.com/dverdonschot/fedora-cosmic-atomic-ewt/actions/workflows/build.yml)

A custom Fedora Atomic image built on COSMIC desktop environment, featuring personal customizations and development tools.

## Features

- **COSMIC Desktop Environment:** Modern, Rust-based desktop from System76
- **Development Tools:** Pre-configured with Rust, Node.js, Python, and more
- **Virtualization Ready:** libvirt, QEMU, virt-manager included
- **AMD GPU Stability Fixes:** Automatic kernel parameters for Radeon graphics
- **COSMIC Vimified:** Keyboard-driven hint navigation (in development)

## Projects in this Repository

### COSMIC Vimified

A keyboard-driven hint navigation system for COSMIC desktop, inspired by the Vimium browser extension. This allows mouseless interaction with GUI elements across your entire desktop.

**Status:** Planning & Specification Phase

See [cosmic-vimified/README.md](cosmic-vimified/README.md) for details.

## Installation

> [!WARNING]  
> [This is an experimental feature](https://www.fedoraproject.org/wiki/Changes/OstreeNativeContainerStable), try at your own discretion.

To rebase an existing atomic Fedora installation to the latest build:

- First rebase to the unsigned image, to get the proper signing keys and policies installed:
  ```
  rpm-ostree rebase ostree-unverified-registry:ghcr.io/dverdonschot/fedora-cosmic-atomic-ewt:latest
  ```
- Reboot to complete the rebase:
  ```
  systemctl reboot
  ```
- Then rebase to the signed image, like so:
  ```
  rpm-ostree rebase ostree-image-signed:docker://ghcr.io/dverdonschot/fedora-cosmic-atomic-ewt:latest
  ```
- Reboot again to complete the installation
  ```
  systemctl reboot
  ```

The `latest` tag will automatically point to the latest build. That build will still always use the Fedora version specified in `recipe.yml`, so you won't get accidentally updated to the next major version.

## ISO

If build on Fedora Atomic, you can generate an offline ISO with the instructions available [here](https://blue-build.org/learn/universal-blue/#fresh-install-from-an-iso). These ISOs cannot unfortunately be distributed on GitHub for free due to large sizes, so for public projects something else has to be used for hosting.

## Post-Installation Configuration

### Rootless Docker (one-time, after rebase)

The image ships with **rootless Docker pre-configured**: `docker-ce-rootless-extras`
is layered on (which provides the per-user binaries, the rootlesskit helpers, and
`dockerd-rootless-setuptool.sh`), `shadow-utils` (for `newuidmap`/`newgidmap`) and
`fuse-overlayfs` are already in the base image, the system `docker.service` and
`docker.socket` are both masked. The rootful `dockerd` cannot start under any
circumstance, and Docker Desktop is intentionally **not** installed (the rootless
`docker-ce` daemon is the only engine).

After a fresh rebase, run **one** command as your normal user (not root):

```bash
/usr/local/bin/setup-rootless-docker.sh
```

The script is idempotent and will:

1. Enable `linger` for your user (so the per-user daemon survives logout).
2. Hand off to `dockerd-rootless-setuptool.sh install` to add your
   `subuid`/`subgid` mapping (it will prompt for sudo — this is expected,
   and only happens on a fresh machine).
3. Generate and enable `~/.config/systemd/user/docker.service`.
4. Start the per-user daemon.
5. Run `verify-rootless-docker.sh` to confirm the final state.

To re-check the state at any time (e.g. after an `rpm-ostree` upgrade):

```bash
/usr/local/bin/verify-rootless-docker.sh
```

If verification ever fails, the script prints a one-line fix for each failed
assertion. The most common regression is `DOCKER_HOST` being set in your shell
init to a rootful path (`unix:///var/run/docker.sock`) — unset it, or set it to
`unix:///run/user/<uid>/docker.sock`.

### AMD GPU Stability (Built-in)

This image includes AMD GPU stability fixes for systems with AMD Radeon integrated graphics (tested on Radeon 780M). The following kernel parameters are automatically applied:

- `amdgpu.gpu_recovery=1` - Enables GPU recovery mechanisms
- `amdgpu.ppfeaturemask=0xffffffff` - Enables all power play features

These fixes address spontaneous reboot issues related to GPU power state restoration failures. No manual configuration is needed.

### Optional: CPU C-State Configuration (AMD CPUs)

If you still experience issues with deep CPU sleep states after the GPU fixes above, you can limit CPU C-states:

```bash
rpm-ostree kargs --append=processor.max_cstate=3
systemctl reboot
```

**Note:** The incorrect Intel-specific parameter `intel_idle.max_cstate=5` should NOT be used on AMD systems. Use `processor.max_cstate` instead, which works on both Intel and AMD CPUs.

This setting persists across updates and only needs to be applied once.

## Verification

These images are signed with [Sigstore](https://www.sigstore.dev/)'s [cosign](https://github.com/sigstore/cosign). You can verify the signature by downloading the `cosign.pub` file from this repo and running the following command:

```bash
cosign verify --key cosign.pub ghcr.io/dverdonschot/fedora-cosmic-atomic-ewt
```
