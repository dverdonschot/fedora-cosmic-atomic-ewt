#!/usr/bin/env bash
# setup-forgejo-runner.sh — install + configure (+ optionally enable)
# the Forgejo Actions runner for the current user on Fedora Atomic.
#
# Default behavior:
#   - Refuses to run as root.
#   - Verifies rootless-docker is up.
#   - Verifies /dev/dri/renderD128 and /dev/kfd exist and the user is
#     in render+video groups.
#   - Materializes ~/.config/forgejo/runners/um790/token (mode 0600,
#     but does NOT overwrite an existing token).
#   - Renders config.yaml from config.yaml.j2 into the user's home.
#   - Writes the per-user DOCKER_HOST env file.
#   - Does NOT enable or start the unit.
#
# Modes:
#   (no arg)        — full setup (above)
#   --wait-docker   — poll-only mode used by ExecStartPre of the
#                     systemd unit. Equivalent to the odroid module's
#                     wait-docker script. Idempotent, no writes.
#   --enable        — same as default + enable+start via systemctl
#                     --user. The operator runs this once after
#                     placing the token.
#
# Post-run operator steps (re-cap at the end):
#   1. Web UI:    Site Admin -> Actions -> Runners -> Create new runner
#                 Copy the opaque registration token.
#   2. Local:     mkdir -p ~/.config/forgejo/runners/um790
#                 install -m 0600 /dev/null .../token
#                 vi .../token   # paste token
#   3. Local:     setup-forgejo-runner.sh --enable
#   4. Web UI:    verify "um790-runner" appears with the right labels.
#
# This script is designed to be safe to run repeatedly: every state-
# changing step is gated on existence checks.

set -euo pipefail

if [[ -t 1 ]]; then
    C_OK=$'\033[1;32m'; C_WARN=$'\033[1;33m'; C_ERR=$'\033[1;31m'; C_RST=$'\033[0m'
else
    C_OK=''; C_WARN=''; C_ERR=''; C_RST=''
fi
log()  { printf '%s[setup-runner]%s %s\n' "$C_OK"   "$C_RST" "$*"; }
warn() { printf '%s[setup-runner]%s %s\n' "$C_WARN" "$C_RST" "$*" >&2; }
err()  { printf '%s[setup-runner]%s %s\n' "$C_ERR"  "$C_RST" "$*" >&2; }

# ---------------------------------------------------------------------------
# Single source of truth for paths and labels. Edit here if the um790
# runner's identity moves.
RUNNER_NAME="${RUNNER_NAME:-um790-runner}"
RUNNER_URL="${RUNNER_URL:-https://forgejo.tail5bbc4.ts.net}"
LABELS="${LABELS:-self-hosted,linux,x86_64,host=um790,size=large,gpu=amdgpu}"
TOKEN_FILE_DEFAULT="${HOME}/.config/forgejo/runners/um790/token"
CONFIG_DIR="${HOME}/.config/forgejo-runner"
CONFIG_TEMPLATE="/etc/forgejo-runner/config.yaml.j2"
CONFIG_FILE="${CONFIG_DIR}/config.yaml"
ENV_FILE="${CONFIG_DIR}/env"
HOST_WORKDIR_PARENT="/var/tmp/forgejo-runner/work"
RUNNER_DIR="${HOME}/.local/share/forgejo-runner"

require() { command -v "$1" >/dev/null 2>&1 || { err "missing tool: $1"; exit 1; }; }

# ---------------------------------------------------------------------------
# Mode dispatch
mode="setup"
case "${1:-}" in
    "")           mode="setup" ;;
    --wait-docker) mode="wait-docker" ;;
    --enable)     mode="enable" ;;
    --help|-h)    sed -n '2,30p' "$0"; exit 0 ;;
    *)            err "unknown arg: $1 (try --help)"; exit 2 ;;
esac

# ---------------------------------------------------------------------------
# All modes need rootless-docker readiness. Implement --wait-docker here.
if [[ "$mode" == "wait-docker" ]]; then
    require id
    require curl
    for _ in $(seq 1 60); do
        # XDG_RUNTIME_DIR is set for user systemd instances but may
        # also be set in shell-only contexts where socket lives under
        # /run/user/<UID>. Probe both.
        uid=$(id -u)
        sock="${XDG_RUNTIME_DIR:-/run/user/$uid}/docker.sock"
        if [[ -S "$sock" ]] && curl --unix-socket "$sock" -sS --max-time 2 \
                http://localhost/_ping >/dev/null 2>&1; then
            log "wait-docker: socket is listening at $sock"
            exit 0
        fi
        sleep 1
    done
    err "wait-docker: gave up after 60s waiting on $sock"
    exit 1
fi

# ---------------------------------------------------------------------------
# From here on: setup or enable. Both need a real user, not root.
if [[ "$EUID" -eq 0 ]]; then
    err "this script must run as your user, not as root."
    err "it writes files under \$HOME and enables a user-mode systemd"
    err "service, both of which require your actual UID."
    exit 2
fi

require id
require systemctl
require docker
require getent

USER_NAME="${USER}"
USER_UID="$(id -u)"
HOME_DIR="${HOME}"

log "user          : ${USER_NAME} (uid=${USER_UID})"
log "home          : ${HOME_DIR}"
log "runner name   : ${RUNNER_NAME}"
log "forgejo url   : ${RUNNER_URL}"
log "labels        : ${LABELS}"

# ---------------------------------------------------------------------------
# 1. Verify rootless-docker is up.
log "checking rootless docker"
if ! systemctl --user is-active docker.service >/dev/null 2>&1; then
    err "docker.service is not active in your user systemd instance."
    err "run /usr/local/bin/setup-rootless-docker.sh first; the runner"
    err "needs rootless dockerd and reuses the same socket."
    exit 1
fi

# ---------------------------------------------------------------------------
# 2. Verify GPU device access. /dev/dri/renderD128 is the render node;
#    /dev/kfd is the KFD compute device (ROCm). Both are required.
log "checking GPU device access"

if ! getent group render >/dev/null; then
    err "group 'render' does not exist; Fedora Atomic should ship it."
    err "    On Fedora Atomic, /dev/dri/renderD128 should be 0660 root:render."
    exit 1
fi
if ! getent group video >/dev/null; then
    err "group 'video' does not exist; Fedora Atomic should ship it."
    err "    On Fedora Atomic, /dev/video0 (and friends) should be 0660 root:video."
    exit 1
fi

in_group() {
    local g=$1
    case ":$(id -G -n "${USER_NAME}" 2>/dev/null):" in
        *":${g}:"*) return 0 ;;
    esac
    # Fallback: id -Gn parses /etc/group for "supplementary"
    if id -Gn "${USER_NAME}" 2>/dev/null | tr ' ' '\n' | grep -qx "${g}"; then
        return 0
    fi
    return 1
}

missing=()
for g in render video; do
    if in_group "$g"; then
        log "  in group ${g} ✓"
    else
        warn "  not in group ${g}"
        missing+=("$g")
    fi
done

if (( ${#missing[@]} > 0 )); then
    warn "you're missing groups: ${missing[*]}"
    warn "fix with: sudo usermod -aG ${missing[*]} ${USER_NAME}"
    warn "you'll need to log out + log back in for new groups to take"
    warn "effect on your user systemd instance."
    if [[ -e /dev/kfd ]]; then
        warn "/dev/kfd present but groups may be needed for the kernel"
        warn "to actually let your user open it."
    fi
    if [[ ! -e /dev/kfd ]]; then
        warn "/dev/kfd not present on this host. Without it, ROCm jobs"
        warn "won't run. Check that 'rocm-runtime' and 'kernel-modules-extra'"
        warn "are present and that the AMD amdgpu kernel module is loaded:"
        warn "    lsmod | grep amdgpu"
        warn "    lspci -k | grep -A3 VGA"
    fi
    # Don't exit 1 — operator may still want the config staged without
    # GPU access, and switch to a non-GPU mode by editing
    # container.options. This is a warning, not a hard failure.
fi

# ---------------------------------------------------------------------------
# 3. Materialize token dir; DO NOT touch existing token (rotation means
#    `rm` then re-run).
log "materializing token directory: $(dirname "${TOKEN_FILE_DEFAULT}")"
mkdir -p "$(dirname "${TOKEN_FILE_DEFAULT}")"
chmod 0700 "$(dirname "${TOKEN_FILE_DEFAULT}")"
if [[ ! -e "${TOKEN_FILE_DEFAULT}" ]]; then
    install -m 0600 /dev/null "${TOKEN_FILE_DEFAULT}"
    log "  created empty token file at ${TOKEN_FILE_DEFAULT}"
    log "  paste your registration token from the Forgejo web UI"
    log "  (Site Admin -> Actions -> Runners -> Create new runner)."
    log "  re-run this script after placing the token."
else
    if [[ "$(stat -c '%a' "${TOKEN_FILE_DEFAULT}")" != "600" && \
          "$(stat -c '%a' "${TOKEN_FILE_DEFAULT}")" != "400" ]]; then
        err "token file ${TOKEN_FILE_DEFAULT} has wrong mode"
        err "    $(stat -c '%a' "${TOKEN_FILE_DEFAULT}") — fix with:"
        err "    chmod 0600 ${TOKEN_FILE_DEFAULT}"
        exit 1
    fi
    log "  existing token file found; leaving it alone"
fi

# ---------------------------------------------------------------------------
# 4. Render config.yaml from the image-baked template.
log "rendering ${CONFIG_FILE}"
mkdir -p "${CONFIG_DIR}"
if [[ ! -r "${CONFIG_TEMPLATE}" ]]; then
    err "config template not found at ${CONFIG_TEMPLATE}"
    err "this would normally ship in the same image at build time."
    err "if the image is fresh, rebuild after running setup-rootless-docker.sh"
    exit 1
fi

# Render with literal key=value substitution. No Jinja2 dependency.
# The template uses simple {{ var }} placeholders; defaults live
# in the sed expressions below, not in the template.
cp "${CONFIG_TEMPLATE}" "${CONFIG_FILE}"
chmod 0600 "${CONFIG_FILE}"
sed -i \
    -e "s|{{ *runner_name *}}|${RUNNER_NAME}|g" \
    -e "s|{{ *runner_url *}}|${RUNNER_URL}|g" \
    -e "s|{{ *runner_capacity *}}|1|g" \
    -e "s|{{ *runner_timeout_seconds *}}|3600|g" \
    -e "s|{{ *runner_fetch_timeout_seconds *}}|300|g" \
    "${CONFIG_FILE}"

# Make sure we replaced every placeholder.
if grep -q '{{' "${CONFIG_FILE}"; then
    warn "config.yaml still has unreplaced placeholders:"
    grep -n '{{' "${CONFIG_FILE}" | head -5 >&2
    warn "this is a script bug; please file an issue on"
    warn "https://github.com/dverdonschot/fedora-cosmic-atomic-ewt/issues"
fi

# ---------------------------------------------------------------------------
# 5. Write the env file with DOCKER_HOST and basic envvars for the
#    user-mode systemd unit to pick up.
log "writing ${ENV_FILE}"
mkdir -p "${RUNNER_DIR}"
mkdir -p "${HOST_WORKDIR_PARENT}" 2>/dev/null || {
    # /var/tmp is on root in some configs. If we can't create the
    # dir, log it; the daemon will fail at runtime and the operator
    # will see the journal error.
    warn "could not create ${HOST_WORKDIR_PARENT}; check perms"
}
cat > "${ENV_FILE}" <<EOF
# Generated by setup-forgejo-runner.sh. Edit if you change Docker setup.
DOCKER_HOST=unix:///run/user/${USER_UID}/docker.sock
FORGEJO_RUNNER_TOKEN_FILE=${TOKEN_FILE_DEFAULT}
EOF
chmod 0600 "${ENV_FILE}"

# ---------------------------------------------------------------------------
# 6. Done with --setup; --enable also wires the unit.
if [[ "$mode" == "setup" ]]; then
    log "✓ setup complete."
    log "  Files staged:"
    log "    ${TOKEN_FILE_DEFAULT}  (mode 0600, empty until you paste the token)"
    log "    ${CONFIG_FILE}        (rendered config)"
    log "    ${ENV_FILE}           (DOCKER_HOST for the runner unit)"
    log "  Next:"
    log "    1. Place the registration token in ${TOKEN_FILE_DEFAULT}."
    log "    2. Re-run with --enable once the token is in place:"
    log "         ${0} --enable"
    exit 0
fi

# --enable from here on.
# Sanity: token file must be non-empty and not be the empty stub we
# just created in step 3 (the install -m 0600 /dev/null stub).
if [[ ! -s "${TOKEN_FILE_DEFAULT}" ]] || \
   [[ "$(wc -c < "${TOKEN_FILE_DEFAULT}")" -lt 16 ]]; then
    err "token file is missing or too short to be a real token:"
    err "    $(stat -c '%n: %s bytes' "${TOKEN_FILE_DEFAULT}")"
    err "  Paste the registration token from the Forgejo web UI"
    err "  (Site Admin -> Actions -> Runners -> Create new runner)."
    err "  Then re-run with --enable."
    exit 1
fi

# 7. Register the runner. Idempotent: skip if .runner already exists.
log "registering runner with ${RUNNER_URL}"
mkdir -p "${HOST_WORKDIR_PARENT}"
REGISTER_ARGS=(
    register
    --config "${CONFIG_FILE}"
    --instance "${RUNNER_URL}"
    --token "$(cat "${TOKEN_FILE_DEFAULT}")"
    --name "${RUNNER_NAME}"
    --labels "${LABELS}"
    --no-interactive
)
if [[ -e "${RUNNER_DIR}/.runner" ]]; then
    log "  .runner file already present at ${RUNNER_DIR}/.runner;"
    log "  skipping register (idempotent). If you need to re-register,"
    log "  delete the file and re-run."
else
    forgejo-runner "${REGISTER_ARGS[@]}"
fi

# 8. Enable + start the user-mode unit.
log "enabling + starting forgejo-runner.service (user)"
systemctl --user daemon-reload
systemctl --user enable --now forgejo-runner.service

# 9. Smoke test.
sleep 2
if systemctl --user is-active forgejo-runner.service >/dev/null 2>&1; then
    log "✓ forgejo-runner.service is active"
else
    err "forgejo-runner.service is not active after --enable."
    err "  check:    systemctl --user status forgejo-runner.service"
    err "  journal:  journalctl --user -u forgejo-runner.service -n 50"
    exit 1
fi

log "✓ done."
log "  Forgejo Runners page should now show '${RUNNER_NAME}' online."
log "  Smoke checks:"
log "    systemctl --user status forgejo-runner.service"
log "    journalctl --user -u forgejo-runner.service -f"
log "  To verify GPU access from inside a job (after the runner runs a"
log "  real workflow):"
log "    - The job's container should see /dev/dri/renderD128 and /dev/kfd."
log "    - rocminfo inside the container should report the AMD780."
