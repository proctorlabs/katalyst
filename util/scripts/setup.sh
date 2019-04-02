#!/bin/bash
ROOT=$(git rev-parse --show-toplevel)
TARGET=${ROOT}/.git/hooks/pre-commit

echo 'Installing cargo-make...'
(cargo install -q cargo-make || true)

echo 'Setting up git commit hooks...'
cat > $TARGET <<EOF
#!/bin/bash
cargo make readme
git add $ROOT/README.md
EOF
chmod +x $TARGET
