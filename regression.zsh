#!/usr/bin/env zsh

for f (*/solution.rs); do
    rustc -C 'opt-level=3' $f -o ${f:r} &
done

wait

for solution (*/solution); do
    output=$($solution)

    if [[ -f ${solution:h}/output.txt ]]; then
        printf '%s' $output | diff -q - ${solution:h}/output.txt
    else
        printf '%s' $output > ${solution:h}/output.txt
    fi
done
