#!/bin/bash

set -e

# setup of tmp directory
x="/tmp/minimize-testcrate"
[ -d "$x" ] && rm -r "$x"
cargo new "$x" --quiet
echo "intrinsics = { path = \"$(pwd)/intrinsics\"}" >> "$x/Cargo.toml"

# actual testing
for i in $(find tests -type f | cut -d "/" -f 2 | cut -d "." -f 1 | sort -h)
do
    echo "========="
    echo "tests/${i}.rs"
    echo "========="
    echo

    res1=$(cargo r --quiet "tests/${i}.rs")
    echo "$res1"
    cp "tests/${i}.rs" "$x/src/main.rs"
    res2=$(cargo run --quiet --manifest-path="$x/Cargo.toml")
    echo "----------"
    echo "$res2"
    if [[ ! "$res1" == "$res2" ]]; then
        echo different output:
        echo "$res1"
        echo "$res2"
        exit
    fi

    echo
    echo
done
