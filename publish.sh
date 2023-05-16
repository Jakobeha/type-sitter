set -e
test -z "$(git status --porcelain)" || (echo "Working directory is not clean" && exit 1)
./test.sh || (echo "Tests failed" && exit 1)
echo "*** PUBLISHING type-sitter-gen"
cargo publish -p type-sitter-gen
echo "*** PUBLISHING type-sitter-lib"
cargo publish -p type-sitter-lib
echo "*** PUBLISHING type-sitter-proc"
cargo publish -p type-sitter-proc
echo "*** PUBLISHING type-sitter-cli"
cargo publish -p type-sitter-cli
echo "*** PUBLISHED"