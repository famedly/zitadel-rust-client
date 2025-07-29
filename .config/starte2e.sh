#!/bin/sh

# SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
#
# SPDX-License-Identifier: Apache-2.0

set -eu

# CI does not add /usr/bin to $PATH for some reason, which means we
# lack docker
export PATH="${PATH}:/usr/bin"

# Make sure the containers can write some files that need to be shared
touch tests/environment/zitadel/service-user.json
chmod a+rw tests/environment/zitadel/service-user.json

# Shut down any still running test-setup first
docker compose --project-directory ./tests/environment down -v || true
docker compose --project-directory ./tests/environment up --wait
