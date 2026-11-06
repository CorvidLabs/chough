# The Linux images Chough's tests run in (REQ-cli-058, DRIVE-22). Build them with
# `fledge run linux-image`, which passes the tool versions the CI workflow pins; this
# file names no tool version itself.

# chough-linux:1: the toolchain, as root.
FROM rust:1.95-bookworm AS toolchain
ARG SPECSYNC_VERSION
ARG FLEDGE_VERSION
ARG HUMAN_INTENT_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git jq python3 procps \
 && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked human-intent --version "${HUMAN_INTENT_VERSION}" \
 && cargo install --locked fledge --version "${FLEDGE_VERSION}" \
 && cargo install --locked specsync --version "${SPECSYNC_VERSION}" \
 && rm -rf /usr/local/cargo/registry
RUN rustup component add rustfmt clippy
RUN git config --global user.name "Chough Linux test" \
 && git config --global user.email linux-test@example.invalid \
 && git config --global --add safe.directory '*' \
 && git config --global init.defaultBranch main
WORKDIR /work

# chough-linux:2: a uid-501 user, matching the macOS account that owns the checkout.
FROM toolchain AS dev
RUN useradd -m -u 501 -s /bin/bash dev && chown -R dev /usr/local/cargo /usr/local/rustup \
 && git config --system --add safe.directory '*'
USER dev
RUN git config --global user.name "Chough Linux test" \
 && git config --global user.email linux-test@example.invalid \
 && git config --global --add safe.directory '*' \
 && git config --global init.defaultBranch main
ENV PATH=/usr/local/cargo/bin:$PATH

# chough-linux:3: adds rsync, for runners that sync a worktree into a volume.
FROM dev AS runner
USER root
RUN apt-get update && apt-get install -y --no-install-recommends rsync \
 && rm -rf /var/lib/apt/lists/*
USER dev
