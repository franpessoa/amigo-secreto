front:
    cd frontend
    yarn
    yarn build
    cd ..

back-debug:
    cargo build

back-release:
    cargo build --release

full: front back-release

full-release: front back-debug


run: full-release
    ./target/release/amigo-secreto

run-debug: full
    ./target/debug/amigo-secreto

podman:
    podman build --tag=amigo-secreto .
    podman run -p 3000:3000 --init amigo-secreto

compose-up:
    podman-compose up

compose-up-d:
    podman-compose up -d

compose-down:
    podman-compose down

compose-build:
    podman-compose build


