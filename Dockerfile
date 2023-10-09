# Build
FROM rust:alpine as build

WORKDIR /build
COPY . .

RUN apk add --no-cache musl-dev upx
RUN cargo b -r
RUN strip target/release/miniqr && upx --best target/release/miniqr

# Deploy
FROM scratch
LABEL maintainer='blek! <me@blek.codes>'

COPY --from=build /build/target/release/miniqr /miniqr

CMD [ "/miniqr" ]