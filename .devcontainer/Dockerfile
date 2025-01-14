FROM mcr.microsoft.com/devcontainers/base:debian-12

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

RUN apt update && \
    apt install -y curl && \
    echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_12/ /' | tee /etc/apt/sources.list.d/shells:fish:release:3.list && \
    curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_12/Release.key | gpg --dearmor | tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg > /dev/null && \
    apt update && \
    apt install -y \
        git \
        xdg-utils \
        build-essential \
        libssl-dev \
        zlib1g-dev \
        libbz2-dev \
        libreadline-dev \
        libsqlite3-dev \
        libncursesw5-dev \
        xz-utils \
        tk-dev \
        libxml2-dev \
        libxmlsec1-dev \
        libffi-dev \
        liblzma-dev \
        fish && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

USER vscode

ENV HOME="/home/vscode"
ENV FISH_CONFIG_DIR="$HOME/.config/fish"
ENV FISH_CONFIG_FILE="$FISH_CONFIG_DIR/config.fish"

RUN mkdir -p "$FISH_CONFIG_DIR"

SHELL ["/usr/bin/fish", "-c"]

ENV RUST_VERSION="1.83.0"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    fish_add_path "$HOME/.cargo/bin" && \
    rustup install "$RUST_VERSION" && \
    rustup default "$RUST_VERSION"

ENV PYTHON_VERSION="3.13.1"
RUN curl -fsSL https://pyenv.run | bash && \
    set -Ux PYENV_ROOT "$HOME/.pyenv" && \
    fish_add_path "$PYENV_ROOT/bin" && \
    echo "pyenv init - fish | source" >> "$FISH_CONFIG_FILE" && \
    pyenv install "$PYTHON_VERSION" && \
    pyenv global "$PYTHON_VERSION"

RUN echo "alias ll='ls -alF'" >> "$FISH_CONFIG_FILE"
RUN echo "functions --erase fish_greeting" >> "$FISH_CONFIG_FILE"
    
WORKDIR /work
