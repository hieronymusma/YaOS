#!/bin/bash -e
declare -a rust_files=()
files=$(git diff-index --name-only HEAD)
echo 'Formatting source files'
for file in $files; do
    if [ ! -f "${file}" ]; then
        continue
    fi
    if [[ "${file}" == *.rs ]]; then
        rust_files+=("${file}")
    fi
done
if [ ${#rust_files[@]} -ne 0 ]; then
    command -v rustfmt >/dev/null 2>&1 || { echo >&2 "Rustfmt is required but it's not installed. Aborting."; exit 1; }
    $(command -v rustfmt) ${rust_files[@]} &
fi