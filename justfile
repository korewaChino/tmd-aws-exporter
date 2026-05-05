image_tag := "ghcr.io/korewachino/tmd-aws-exporter:latest"

build:
    podman build -t {{ image_tag }} .

push:
    podman push {{ image_tag }}

run:
    podman run {{ image_tag }}
