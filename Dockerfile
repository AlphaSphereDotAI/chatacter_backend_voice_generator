FROM ghcr.io/prefix-dev/pixi:noble-cuda-12.6.3

SHELL ["/bin/bash", "-c"]

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive \
    UV_NO_CACHE=true \
    PATH="/root/.pixi/bin:${PATH}"

RUN apt-get update && \
    apt-get full-upgrade -y && \
    apt-get autoremove && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY pyproject.toml .
COPY .python-version .

RUN pixi global install uv && \
    uv python install $(cat .python-version) && \
    uv sync && \
    uv run huggingface-cli download suno/bark-small --quiet

COPY . .

RUN rm -rf .git && \
    rm -rf .github && \
    rm -rf .gitignore && \
    rm -rf .deepsource.toml && \
    rm -rf .python-version && \
    rm -rf Dockerfile && \
    rm -rf pyproject.toml && \
    rm -rf README.md && \
    rm -rf renovate.json && \
    rm -rf .uv.lock && \
    rm -rf .flox 

EXPOSE 8001

CMD ["uv", "run", "fastapi", "dev", "--host", "0.0.0.0", "--port", "8001"]
