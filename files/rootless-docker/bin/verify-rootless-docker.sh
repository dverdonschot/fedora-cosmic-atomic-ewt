#!/usr/bin/env bash
# verify-rootless-docker.sh — assert that the per-user rootless Docker
# daemon is actually live, on this user, in this session. Exits non-zero
# on the first failure with a one-line diagnostic.
#
# Use this:
#   * after running setup-rootless-docker.sh for the first time
#   * after any rpm-ostree upgrade that touched docker-ce
#   * after `systemctl --user reset-failed` or other systemd janitorial work
#   * in a bug report: paste the output and we can see what's wrong
#
# Does NOT modify state. Pure read-only check.

set -uo pipefail

if [[ -t 1 ]]; then
    C_OK=$'\033[1;32m'; C_FAIL=$'\033[1;31m'; C_INFO=$'\033[1;36m'; C_RST=$'\033[0m'
else
    C_OK=''; C_FAIL=''; C_INFO=''; C_RST=''
fi
log()    { printf '%s[verify]%s %s\n' "$C_INFO"  "$C_RST" "$*"; }
pass()   { printf '  %sPASS%s  %s\n' "$C_OK"  "$C_RST" "$*"; }
fail()   { printf '  %sFAIL%s  %s\n' "$C_FAIL" "$C_RST" "$*" >&2; }

require() {
    command -v "$1" >/dev/null 2>&1 || {
        fail "missing required tool: $1"; exit 1;
    }
}

# Refuse to run as root: a root user is by definition not using a
# per-user rootless daemon; this script is only meaningful for real users.
if [[ "$EUID" -eq 0 ]]; then
    fail "this script must run as your user, not as root."
    fail "rootless Docker is a per-user setup; root has a different code path."
    exit 2
fi

require docker
require systemctl

USER_NAME="${USER}"
USER_UID="$(id -u)"
XDG_RT="${XDG_RUNTIME_DIR:-/run/user/${USER_UID}}"
SOCKET="${XDG_RT}/docker.sock"

failed=0

# --- Assertion 1: per-user systemd instance is running -------------------
# The unit lives in the user instance. If the instance is down (common
# right after a fresh login, or in headless contexts without linger),
# every later check is going to lie.
log "1/5 user systemd instance is running"
if systemctl --user is-system-running >/dev/null 2>&1 \
   || systemctl --user is-active docker.service >/dev/null 2>&1; then
    pass "systemd --user is up"
else
    # `is-system-running` returns "offline" before the user has any
    # active units. Treat that as "not really up yet" but recoverable.
    if systemctl --user status >/dev/null 2>&1; then
        pass "systemd --user is reachable (no active units yet)"
    else
        fail "systemd --user is not reachable. try: loginctl enable-linger $USER_NAME"
        failed=1
    fi
fi

# --- Assertion 2: per-user docker.service is active -----------------------
log "2/5 docker.service is active in the user instance"
state="$(systemctl --user is-active docker.service 2>/dev/null || echo unknown)"
case "${state}" in
    active) pass "docker.service is active" ;;
    *)
        fail "docker.service state: ${state}"
        fail "fix: systemctl --user enable --now docker.service"
        failed=1
        ;;
esac

# --- Assertion 3: socket exists where rootless puts it -------------------
log "3/5 ${SOCKET} exists"
if [[ -S "${SOCKET}" ]]; then
    pass "socket present"
else
    fail "no socket at ${SOCKET}"
    fail "this is the user-scoped runtime dir; if it's missing, the"
    fail "daemon never opened it. check 'journalctl --user -u docker.service'."
    failed=1
fi

# --- Assertion 4: docker CLI talks to the daemon and reports rootless ----
log "4/5 docker info reports the rootless daemon"
if ! info="$(docker info 2>&1)"; then
    fail "docker info failed. raw output:"
    printf '%s\n' "${info}" | sed 's/^/    /' >&2
    fail "common cause: the active context points at a socket that does"
    fail "not exist. try: docker context use rootless"
    failed=1
else
    # The rootless indicator in `docker info` is the single word
    # `rootless` appearing as a value under the `Security Options:`
    # section (one indent level under the `Server:` block, two under
    # the indented `Security Options:` header). NOT a key/value pair
    # like `Rootless: true` -- that field does not exist in stock
    # `docker info` output. Match the actual format.
    if grep -qE '^Server:|^[^ ]' <<<"${info}"; then
        :  # the Server: block exists; below checks will be meaningful
    fi
    if awk '
        /^Server:/                       { in_server = 1; next }
        in_server && /^[^ ]/             { in_server = 0 }
        in_server && /^ Security Options:/ { in_sec = 1; next }
        in_sec && /^[^ ]/                { in_sec = 0 }
        in_sec && /^[ \t]+rootless[ \t]*$/ { found = 1 }
        END { exit found ? 0 : 1 }
    ' <<<"${info}"; then
        pass "docker info: Security Options includes 'rootless'"
    else
        # Surface enough context to diagnose without dumping the whole
        # info blob. The Server Version line proves the client reached
        # a daemon at all; the active context name tells us which one.
        context="$(grep -E '^ Context:' <<<"${info}" || echo '(no client context line)')"
        server="$(grep -E '^ Server Version:' <<<"${info}" || echo '(no server version)')"
        secopts="$(grep -E '^ Security Options:' <<<"${info}" || echo '(no security options line)')"
        fail "docker info succeeded but no rootless indicator under Security Options."
        fail "  ${context}"
        fail "  ${server}"
        fail "  ${secopts}"
        fail "this usually means the client is talking to a non-rootless"
        fail "daemon. check: docker context show"
        failed=1
    fi
fi

# --- Assertion 5: subuid/subgid are set for this user ---------------------
# Without these, every bind-mount from inside a container fails with
# newuidmap EPERM and the user has no clue why.
log "5/5 subuid/subgid contain an entry for ${USER_NAME}"
subuid_ok=0; subgid_ok=0
if grep -qE "^${USER_NAME}:" /etc/subuid 2>/dev/null; then
    subuid_ok=1
fi
if grep -qE "^${USER_NAME}:" /etc/subgid 2>/dev/null; then
    subgid_ok=1
fi
if [[ "${subuid_ok}" -eq 1 && "${subgid_ok}" -eq 1 ]]; then
    pass "subuid and subgid both have an entry for ${USER_NAME}"
else
    if [[ "${subuid_ok}" -eq 0 ]]; then
        fail "/etc/subuid has no entry for ${USER_NAME}"
    fi
    if [[ "${subgid_ok}" -eq 0 ]]; then
        fail "/etc/subgid has no entry for ${USER_NAME}"
    fi
    fail "fix: run /usr/local/bin/setup-rootless-docker.sh — it will"
    fail "     hand off to dockerd-rootless-setuptool.sh which writes"
    fail "     these files (needs sudo, one-time)."
    failed=1
fi

echo
if [[ "${failed}" -ne 0 ]]; then
    fail "one or more checks failed; rootless is NOT fully configured."
    exit 1
fi
log "all checks passed; rootless Docker is healthy for ${USER_NAME}."
