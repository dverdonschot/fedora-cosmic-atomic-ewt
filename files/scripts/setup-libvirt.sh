#!/usr/bin/env bash

# Tell this script to exit if there are any errors.
set -oue pipefail

echo "Setting up libvirt and qemu user groups..."

# Create qemu user and group if they don't exist
# The qemu user is a system account used by libvirtd to run VMs
if ! getent group qemu > /dev/null 2>&1; then
    echo "Creating qemu group (GID 107)..."
    groupadd -r -g 107 qemu || groupadd -r qemu
fi

if ! getent passwd qemu > /dev/null 2>&1; then
    echo "Creating qemu user (UID 107)..."
    useradd -r -g qemu -u 107 -s /sbin/nologin -c "qemu user" -d / qemu || \
    useradd -r -g qemu -s /sbin/nologin -c "qemu user" -d / qemu
fi

# Create kvm group if it doesn't exist (for /dev/kvm access)
if ! getent group kvm > /dev/null 2>&1; then
    echo "Creating kvm group..."
    groupadd -r kvm
fi

# Add qemu user to kvm group (so VMs can use hardware acceleration)
usermod -aG kvm qemu || true

# Add all regular users to libvirt and kvm groups
# This allows them to use virt-manager and access /dev/kvm
for user in $(getent passwd | awk -F: '$3 >= 1000 && $3 < 65534 {print $1}'); do
    echo "Adding user $user to libvirt and kvm groups"
    usermod -aG libvirt,kvm "$user" || true
done

echo "Libvirt setup complete!"
