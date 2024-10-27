FROM rust:latest

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
    sudo \
    && rm -rf /var/lib/apt/lists/*

# ユーザーとグループの作成
RUN groupadd -g ${GROUP_ID} ${USER_NAME} \
    && useradd -m -u ${USER_ID} -g ${GROUP_ID} ${USER_NAME} \
    && echo ${USER_NAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USER_NAME}\
    && chmod 0440 /etc/sudoers.d/${USER_NAME} 

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME} \
    && echo 'export PS1="\e[01;32m\u\e[m:\e[01;34m\w\e[m\$ "' >> /home/${USER_NAME}/.bashrc

# Rustのインストール
RUN rustup target add wasm32-unknown-unknown 

# dioxusのCLIを入れておく
RUN cargo install dioxus-cli --force \ 
    && cargo install tauri-cli \
    && cargo install create-tauri-app --locked \
    && cargo install -f wasm-bindgen-cli --version 0.2.95

# 権限周りの解決がしたい
USER ${USER_NAME}
RUN sudo chmod -R 777 /usr/local/cargo

# シェルの設定
SHELL ["/bin/bash", "-c"]