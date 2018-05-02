# term-status

Because when else am I going to use `git2`?

## What I replaced

```fish
function parse_git_branch_and_add_brackets
  git branch --no-color 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/\[\1\]/'
end

function fish_prompt
  printf "%s => " (bold)(parse_git_branch_and_add_brackets)(normal)
end
```

## What it is now

```fish
function fish_prompt
  term-status
end
```
