#!/usr/bin/env bash

# Tell this script to exit if there are any errors.
set -oue pipefail

echo "Setting up libvirt user groups..."

# Add all users with UID >= 1000 to libvirt group
# This allows them to use virt-manager without sudo
for user in $(getent passwd | awk -F: '$3 >= 1000 && $3 < 65534 {print $1}'); do
    echo "Adding user $user to libvirt group"
    usermod -aG libvirt "$user" || true
done

echo "Libvirt setup complete!"
