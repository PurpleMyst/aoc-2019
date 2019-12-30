#!/usr/bin/env zsh

setopt ERR_EXIT NO_UNSET PIPE_FAIL

printf '' > all.rs

for f (*/solution.rs); do
    printf 'mod day%s {\n' ${f:h} >> all.rs
    sed -e 's/fn main/pub \0/' -e 's_\.\./intcode.rs_intcode.rs_' -e "s_\w\+.txt_${f:h}/\0_" $f >> all.rs
    printf '}\n' >> all.rs
done

printf 'fn main() {\n' >> all.rs
for f (*/*.rs); do
    printf '  day%s::main();\n' ${f:h} >> all.rs
done
printf '}\n' >> all.rs

rustc --allow dead_code -C opt-level=${1:-3} -C target-cpu=native all.rs -o all

hyperfine ./all

rm all.rs all
