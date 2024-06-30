#!/bin/env bash

set -euo pipefail

# Create local registry
k3d registry create flowcar-registry.localhost --port 5000 2>/dev/null || true

# Create cluster
k3d cluster create flowcar --registry-use k3d-flowcar-registry.localhost:5000 2>/dev/null || true

# Build and push docker images
docker build -t k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest webapp/ && docker push k3d-flowcar-registry.localhost:5000/fraguinha/flowcar-webapp:latest

# Configure secret variables
POSTGRES_DB=${POSTGRES_DB:-flowcar_db}
POSTGRES_USER=${POSTGRES_USER:-flowcar_user}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-flowcar_password}

# Create kubernetes secret
kubectl create secret generic secrets \
	--from-literal=POSTGRES_DB="$POSTGRES_DB" \
	--from-literal=POSTGRES_USER="$POSTGRES_USER" \
	--from-literal=POSTGRES_PASSWORD="$POSTGRES_PASSWORD" 2>/dev/null || true

# Apply kubernetes manifests
bash apply.sh
