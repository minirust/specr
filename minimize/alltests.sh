#!/bin/bash

set -e

for i in $(find tests -type f | cut -d "/" -f 2 | cut -d "." -f 1 | sort -h)
do
    echo "========="
    echo "tests/${i}.rs"
    echo "========="
    echo

    res1=$(cargo r --quiet "tests/${i}.rs")
    echo "$res1"

    echo "----------"

    rustc "tests/${i}.rs" -o out -L ./intrinsics/target/debug -l intrinsics -Zalways-encode-mir -Zmir-emit-retag -Zmir-opt-level=0 --cfg=miri -Zextra-const-ub-checks -Cdebug-assertions=off
    res2=$(./out)
    rm ./out

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
