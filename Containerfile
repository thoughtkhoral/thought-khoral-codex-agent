# SPDX-License-Identifier: Apache-2.0
FROM docker.io/library/rust:1.93-bullseye@sha256:fae8ebcb8eda28d37df2b965c87c15a572dac2b6d3893a72d6dea99d700d1786 AS build
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY contracts ./contracts
RUN cargo build --locked --release --bin thought-khoral-codex-agent
RUN set -eu; case "$(uname -m)" in \
    aarch64) arch=aarch64; sum=883620139925f677e5a12c95ba87a1d7f421388ebb797b1c10aba42a04ede9d7;; \
    x86_64) arch=x86_64; sum=306865417d4ee7a927785852910a527f41e1e159add390ac5ae3accb67d44a13;; \
    *) exit 1;; esac; \
    curl --fail --location --proto '=https' --tlsv1.2 "https://github.com/openai/codex/releases/download/rust-v0.160.0/codex-${arch}-unknown-linux-musl.tar.gz" -o /tmp/codex.tar.gz; \
    printf '%s  /tmp/codex.tar.gz\n' "$sum" | sha256sum -c -; \
    tar -xzf /tmp/codex.tar.gz -C /usr/local/bin; \
    mv "/usr/local/bin/codex-${arch}-unknown-linux-musl" /usr/local/bin/codex; \
    mkdir -p /tmp/native /tmp/generated; \
    HOME=/tmp/native CODEX_HOME=/tmp/native codex --version | grep -x 'codex-cli 0.160.0'; \
    HOME=/tmp/native CODEX_HOME=/tmp/native codex app-server generate-json-schema --out /tmp/generated; \
    printf '81a88c04ae4984b16d73080f4109d0477682bc76c8adbe483e371175ce54c054  /tmp/generated/codex_app_server_protocol.v2.schemas.json\n' | sha256sum -c -; \
    printf '62ad689c2cb6379913c1d72749cfd8de5089d35760214123518eb92eef11acc9  /tmp/generated/v1/InitializeResponse.json\n' | sha256sum -c -
FROM docker.io/library/debian:bookworm-slim@sha256:3783cc01769c7b2b1b83a5c5ad96c815348e28ed7da68e2e3687004faa906251
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10003 codex-worker && useradd --uid 10003 --gid 10003 --no-create-home --home-dir /var/lib/thought-khoral-codex/native codex-worker \
    && install -d -m 700 -o 10003 -g 10003 /var/lib/thought-khoral-codex/native /var/lib/thought-khoral-codex/receipts \
    && install -d -m 555 /opt/thought-khoral-codex/workspace /usr/share/licenses/thought-khoral-codex-agent
COPY --from=build /build/target/release/thought-khoral-codex-agent /usr/local/bin/
COPY --from=build /usr/local/bin/codex /usr/local/bin/
COPY LICENSE NOTICE THIRD_PARTY_NOTICES.md /usr/share/licenses/thought-khoral-codex-agent/
COPY licenses/ /usr/share/licenses/thought-khoral-codex-agent/dependencies/
COPY contracts/codex-app-server-0.160.0/LICENSE contracts/codex-app-server-0.160.0/NOTICE /usr/share/licenses/thought-khoral-codex-agent/codex/
COPY contracts/codex-app-server-0.160.0/restricted-models.json contracts/codex-app-server-0.160.0/tool-controls.json contracts/codex-app-server-0.160.0/tool-policy-proof-aarch64.json /opt/thought-khoral-codex/
WORKDIR /opt/thought-khoral-codex/workspace
USER 10003:10003
EXPOSE 9091
ENTRYPOINT ["/usr/local/bin/thought-khoral-codex-agent"]
