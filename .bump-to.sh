#!/usr/bin/env bash

help () {
    echo "Bump project's version"
    echo "Usage:"
    echo "    ./bump-to.sh <options> <version>"
    echo "Params:"
    echo "    version -- new version"
    echo "Options:"
    echo "    -h|--help -- print help"
    echo "    -v|--version -- print current version"
}

check_workspace() {
    if [[ -n "$(git status --porcelain)" ]]; then
        echo "🧹 Workspace isn't clean. Please stash/commit changes before bumping the version."
        exit 1
    fi
}

log_info() {
    echo -e "\033[32mINFO: \033[0m $1"
}

log_error() {
    echo -e "\033[31mERROR: \033[0m $1"
}

warn() {
    echo -e "\033[33m $1 \033[0m"
}


############ start script ############
for arg in "$@"; do
    case $arg in 
        -h|--help)
            help
            exit 0
            ;;
        -v|--version)
            echo "$(git describe --tags --abbrev=0)"
            exit 0
            ;;
        *)
            ;;
    esac
done

check_workspace

old_version=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
new_version=$1

if [[ -z "$new_version" ]]; then
    log_error "Missing version argument."
    exit 1
fi

if ! [[ "$new_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid semver $new_version (expected MAJOR.MINOR.PATCH)."
    exit 1
fi

if git rev-parse v${new_version} > /dev/null 2>&1; then
    log_error "Tag ${new_version} already exists"
    exit 1
fi

log_info "found version $old_version"
log_info "bump to version $new_version"

sed -i '' "s/version = \"$old_version\"/version = \"$new_version\"/g" Cargo.toml
git diff
warn "Please verify the diff. Do you want to continue? [y/N]"
read -r continue
if [[ "$continue" != "y" ]]; then
    echo "Aborted."
    exit 1
fi
# Ensure Cargo.lock matches new version BEFORE git add
cargo check --quiet
git add Cargo.toml Cargo.lock
git commit -m "chore(version): bump version to ${new_version}"
git tag v$new_version
git push
git push origin $new_version

