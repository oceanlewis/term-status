# term_status

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
  term_status
end
```

## What I learned

Okay, so just for fun I wrote a simple benchmark using `test` which, while not exactly scientific, showed me something interesting. The cost of opening up a new process (`term_status`) to do something as mundane as printing a terminal prompt is significantly higher than just letting my shell do it itself. I didn't investigate whether this is specific to Rust, but I image that it's not and instead it's just the cost of doing business with processes.

## Continuing Forward

So, turns out this works great for my other project Dingus, since I can check for a variable I set there and change my prompt accordingly, depending on whether or not I'm currently in a nested shell. Woo!
