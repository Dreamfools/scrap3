set -e

BUILD_DIR="debug"

# Check if the --release argument was passed
if [[ "$1" == "--release" ]]; then
    RELEASE_FLAG="--release"
    BUILD_DIR="release"
fi

cargo lbuild $RELEASE_FLAG --target wasm32-unknown-unknown -p game

rm -r target/web || true

mkdir -p target/web

cp -r game/js/* target/web
cp target/wasm32-unknown-unknown/${BUILD_DIR}/game.wasm target/web

cd target/web

simple-http-server -i