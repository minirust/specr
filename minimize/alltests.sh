#!/bin/bash

set -e

x="/tmp/tmp.minimize-tests"
[ -d "$x" ] && rm -r "$x"
mkdir $x

for i in $(find tests -type f | cut -d "/" -f 2 | cut -d "." -f 1 | sort -h)
do
    echo "========="
    echo "tests/${i}.rs"
    echo "========="
    echo

    res1=$(cargo r --quiet "tests/${i}.rs")
    echo "$res1"
    (
      cd $x;
      cargo new testcrate --quiet;
    )
    x="$x/testcrate"
    echo "intrinsics = { path = \"$(pwd)/intrinsics\"}" >> "$x/Cargo.toml"
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
