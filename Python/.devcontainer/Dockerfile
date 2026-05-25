# https://www.debian.org/releases/
# https://devguide.python.org/versions/#versions
# https://mcr.microsoft.com/en-us/artifact/mar/devcontainers/python/tag/3.14-trixie
# TODO Upgrade minor release 3.15-trixie https://www.debian.org/News/2026/20260516
FROM mcr.microsoft.com/devcontainers/python:3.14-trixie

# OS deps you likely need for builds
RUN apt-get update && apt-get install -y --no-install-recommends \
    git build-essential curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Non-root user
RUN if ! id -u vscode >/dev/null 2>&1; then \
    useradd -m -u 1000 -s /bin/bash vscode; \
    mkdir -p /home/vscode; \
    chown -R vscode:vscode /home/vscode; \
  fi
USER vscode

WORKDIR /workspaces

RUN python -m pip install --upgrade pip && \
    pip check

# Per-user virtualenv as default interpreter
ENV VIRTUAL_ENV=/home/vscode/.venv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Copy the script into the image and execute it as the vscode user
# Note: We create the path to match your request, though copying to /tmp is also common
COPY --chown=vscode:vscode .devcontainer/skel.sh /home/vscode/skel.sh
RUN chmod +x /home/vscode/skel.sh && \
    echo "/home/vscode/skel.sh" >> /home/vscode/.bashrc
