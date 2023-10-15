FROM rust:bookworm as rust-build
WORKDIR /usr/src/

# Dummy project for dependency caching
RUN USER=root cargo new --bin amigo-secreto
COPY Cargo.toml Cargo.lock /usr/src/amigo-secreto/

WORKDIR /usr/src/amigo-secreto/

RUN cargo build --release
RUN rm src/*.rs



# Copy project files
COPY ./src ./src

RUN cargo build --release
#RUN rm ./target/release/deps/amigo-secreto*

# Build frontend
FROM node:bookworm as frontent-build
WORKDIR /usr/src/amigo-secreto/frontend
COPY ./frontend/package.json ./frontend/yarn.lock ./


RUN yarn install

COPY ./frontend .
RUN yarn build

# Run
FROM rust:slim-bookworm
WORKDIR /
COPY --from=rust-build /usr/src/amigo-secreto/target/release/amigo-secreto /
COPY --from=frontent-build /usr/src/amigo-secreto/public/ /
RUN ls
CMD ["./amigo-secreto"]


