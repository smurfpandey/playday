FROM rust:1.53-buster as planner
WORKDIR app
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.53-buster as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53-buster as builder

ENV LANG C.UTF-8
ENV LC_ALL C.UTF-8

RUN apt-get update
RUN apt-get install -y apt-transport-https
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    postgresql-client \
    libpq-dev \
    git \
    curl \
    gcc \
    make \
    openssl \
    libssl-dev

WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

##########################################
# Build frontend
##########################################
FROM node:14-slim as frontend-build

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

FROM debian:buster-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 libssl1.1

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER

# Create and switch to a new user
WORKDIR /usr/app

COPY --from=builder ./app/target/release/playday_web ./playday_web
COPY --from=builder ./app/target/release/playday_celery_beat ./playday_celery_beat
COPY --from=builder ./app/target/release/playday_celery_worker ./playday_celery_worker
COPY --from=frontend-build ./playday_web/static/dist ./static/dist
COPY ./playday_web/templates ./templates
COPY ./diesel.toml .
COPY ./migrations ./migrations

RUN chown -R $APP_USER:$APP_USER .
USER $APP_USER

CMD ["./playday_web"]
