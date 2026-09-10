FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# ==========================================
# 1. Dependências de sistema (root)
# ==========================================
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
        ripgrep \
        fd-find \
        bat \
        hyperfine \
        valgrind \
        lldb \
        git-lfs \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && ln -s /usr/bin/fdfind /usr/local/bin/fd \
    && ln -s /usr/bin/batcat /usr/local/bin/bat

# Registra os filtros smudge/clean do Git LFS globalmente (/etc/gitconfig),
# resolvendo o erro "git-lfs was not found on your path" no pre-push hook.
RUN git lfs install --system

# Cria os diretórios que serão montados como volume JÁ com o dono certo.
RUN mkdir -p /usr/local/cargo/registry /usr/local/cargo/sccache /home/vscode/target-cache \
    && chown -R vscode:vscode /usr/local/cargo/registry /usr/local/cargo/sccache /home/vscode/target-cache

# ==========================================
# 2. Toolchain Rust (vscode)
# ==========================================
USER vscode

RUN rustup component add rustfmt clippy rust-src

# cargo-binstall via script oficial = binário pré-compilado, nunca "cargo install"
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

# --disable-strategies compile = FALHA se não achar binário, em vez de compilar
# escondido. Garante "100% binário" para todo o grupo abaixo.
RUN cargo binstall -y --locked --disable-strategies compile \
        cargo-nextest \
        cargo-edit \
        cargo-watch \
        cargo-expand \
        cargo-outdated \
        cargo-audit \
        cargo-machete \
        cargo-deny \
        cargo-semver-checks \
        cargo-llvm-lines \
        cargo-tarpaulin \
        sccache \
        git-delta \
        just \
        bacon

# taplo-cli não publica binário pré-compilado (nem GitHub Releases nem
# QuickInstall) para esta versão/plataforma — sem --disable-strategies aqui,
# então o binstall cai para compilar da fonte só para este crate. É rápido
# (crate pequeno, sem árvore de deps pesada), então não compromete o
# objetivo de "evitar compilação demorada" do restante do setup.
RUN cargo binstall -y --locked taplo-cli

# ==========================================
# 3. Variáveis de ambiente globais
# ==========================================
ENV RUST_BACKTRACE=1 \
    CARGO_TERM_COLOR=always \
    RUSTC_WRAPPER=sccache \
    CARGO_TARGET_DIR=/home/vscode/target-cache