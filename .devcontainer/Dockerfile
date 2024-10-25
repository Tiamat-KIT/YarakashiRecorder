FROM mcr.microsoft.com/devcontainers/base:ubuntu

# ユーザー設定の引数
ARG USER_NAME=utakata
ARG USER_ID=1100
ARG GROUP_ID=1100

# 環境変数の設定
ENV DEBIAN_FRONTEND=noninteractive

# システムのアップデートと必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsoup2.4-dev \
    unzip \
    javascriptcoregtk-4.1 \
    libsoup-3.0  \
    webkit2gtk-4.1 \
    && rm -rf /var/lib/apt/lists/*

# ユーザーとグループの作成
RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${GROUP_ID} ${USER_NAME}

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME} \
    && echo 'export PS1="\e[01;32m\u\e[m:\e[01;34m\w\e[m\$ "' >> /home/${USER_NAME}/.bashrc

# ユーザーの切り替え
USER ${USER_NAME}
WORKDIR /home/${USER_NAME}

# Bunのインストール (ユーザー専用)
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/home/${USER_NAME}/.bun/bin:${PATH}"
RUN echo 'export PATH="/home/${USER_NAME}/.bun/bin:$PATH"' >> /home/${USER_NAME}/.bashrc

# Rustのインストール (ユーザー専用)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/${USER_NAME}/.cargo/bin:${PATH}"
RUN echo 'export PATH="/home/${USER_NAME}/.cargo/bin:$PATH"' >> /home/${USER_NAME}/.bashrc

# dioxusのCLIを入れておく
RUN cargo install dioxus-cli --locked \ 
    && cargo install tauri-cli --version "^2.0.0" --locked \
    && cargo install create-tauri-app --locked \
    && cargo install -f wasm-bindgen-cli --version 0.2.95 \
    && cargo install dioxus-cli --force
# シェルの設定
SHELL ["/bin/bash", "-c"]