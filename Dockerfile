FROM rust:1.47.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/vader_sentiment_api
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/vader_sentiment_api /usr/local/bin/vader_sentiment_api

CMD ["vader_sentiment_api"]