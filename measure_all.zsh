#!/usr/bin/env zsh

for f (*/*.rs); do
    rustc -C 'opt-level=3' $f -o ${f:r} &
done

wait

time ( for f (*/solution); $f &> /dev/null )
