#!/usr/bin/env fish

echo 'Fish Prompt'
time -p fish -c 'for i in (seq 1 1000); printf "%s => " (bold)(parse_git_branch_and_add_brackets)(normal) > /dev/null; end'

echo

echo 'Rust Prompt'
time -p fish -c 'for i in (seq 1 1000); ./target/release/term-status > /dev/null; end'
