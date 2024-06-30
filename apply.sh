#!/bin/env bash

set -euo pipefail

# Apply manifests
kubectl apply -f k8s/database/persistent-volume.yaml
kubectl apply -f k8s/database/persistent-volume-claim.yaml
kubectl apply -f k8s/database/postgresql-deployment.yaml
kubectl apply -f k8s/database/postgresql-service.yaml
kubectl apply -f k8s/database/postgresql-config.yaml
kubectl apply -f k8s/database/postgresql-job.yaml

kubectl apply -f k8s/webapp/flowcar-deployment.yaml
kubectl apply -f k8s/webapp/flowcar-service.yaml
kubectl apply -f k8s/webapp/flowcar-ingress.yaml
