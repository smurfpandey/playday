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

RUN USER=root cargo new --bin playday
WORKDIR ./playday
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/playday*
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

COPY --from=builder ./playday/target/release/playday ./playday
COPY --from=frontend-build ./static/dist ./static/dist
COPY ./templates ./templates
COPY ./diesel.toml .

RUN chown -R $APP_USER:$APP_USER .
USER $APP_USER


CMD ["./playday"]
