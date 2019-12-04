#!/usr/bin/env zsh

set -xeuo pipefail

for f (**/*.rs)
    rustc -C 'opt-level=3' $f -o ${f:r}

time ( for f (**/solution); $f )
