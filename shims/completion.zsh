# dot zsh completion (sourceable)

autoload -Uz compinit && compinit -i
zmodload zsh/complist 2>/dev/null   # menu-select UI

# Pretty headers & menu
zstyle ':completion:*:*:dot:*' group-name ''
zstyle ':completion:*:descriptions' format '%F{green}-- %d --%f'
zstyle ':completion:*' menu select
# Bias toward one-per-line:
zstyle ':completion:*' list-rows-first 'no'
zstyle ':completion:*' list-packed 'no'

_dot() {
  emulate -L zsh

  local -a lines pairs
  local curfrag=${words[$CURRENT]}

  # ask your CLI; expects "name<TAB>desc" per line (desc optional)
  lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}")
  (( $#lines )) || return 1

  local line name desc
  for line in $lines; do
    name=${line%%$'\t'*}
    if [[ $line == *$'\t'* ]]; then
      desc=${line#*$'\t'}
      pairs+="$name: -- ${desc}"     # render as: name  -- desc
    else
      pairs+="$name"
    fi
  done

  # `_describe` ensures the *name* is inserted, desc shown to the right
  _describe -t dotcmds 'dot commands' pairs
}

compdef _dot dot
