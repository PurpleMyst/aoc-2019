#!/usr/bin/env zsh

set -xeuo pipefail

for f (**/*.rs); do
    rustc -C 'opt-level=3' $f -o ${f:r} &
done

wait

time ( for f (**/solution); $f )
