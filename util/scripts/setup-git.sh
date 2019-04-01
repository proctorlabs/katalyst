#!/bin/bash
ROOT=$(git rev-parse --show-toplevel)
DOCS_SH=${ROOT}/util/scripts/update-docs.sh
TARGET=${ROOT}/.git/hooks/pre-commit

cat > $TARGET <<EOF
#!/bin/bash
$DOCS_SH
EOF

chmod +x $TARGET
