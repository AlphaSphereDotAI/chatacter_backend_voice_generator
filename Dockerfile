FROM docker.io/rust:slim AS build

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /build

COPY . .

# skipcq: DOK-DL3008
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends alsa-utils build-essential clang cmake lbzip2 libasound2-dev llvm; \
    apt-get autoremove; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*; \
    cargo build --release; \
    mkdir -p ./tmp; \
    cp -r ./target/release/** ./tmp

ADD https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/kokoro-en-v0_19.tar.bz2 ./kokoro-en-v0_19.tar.bz2

RUN echo "d21f1843ead42c1b036d2a164777596a9235e1b02f464b8f3d322972b5372b85  ./kokoro-en-v0_19.tar.bz2" | sha256sum -c -; \
    tar xf ./kokoro-en-v0_19.tar.bz2

FROM docker.io/debian:stable-slim AS prod

SHELL ["/bin/bash", "-c"]

WORKDIR /app

RUN set -eux; \
    addgroup --system chatacter \
    && adduser --system chatacter --ingroup chatacter

## copy the main binary
COPY --from=build /build/tmp/** ./
## copy the model## copy the model
COPY --from=build /build/kokoro-en-v0_19 ./kokoro-en-v0_19

USER chatacter

CMD ["./voice_generator"]
