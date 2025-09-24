# dot zsh completion (sourceable, scoped)

# Initialize only if completion isn't already loaded
if ! typeset -p _comps &>/dev/null; then
  autoload -Uz compinit && compinit -i
fi
zmodload -i zsh/complist 2>/dev/null

# ----- SCOPED styles (only when completing `dot`) -----
zstyle ':completion:*:*:dot:*' group-name ''
zstyle ':completion:*:*:dot:*' menu select
zstyle ':completion:*:*:dot:*' list-rows-first 'no'
zstyle ':completion:*:*:dot:*' list-packed 'no'
zstyle ':completion:*:*:dot:*:descriptions' format '%F{green}-- %d --%f'

_dot() {
  emulate -L zsh

  local -a lines pairs
  local header='' line insert display
  local curfrag=${words[$CURRENT]}

  lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}") || return 1
  (( $#lines )) || return 1

  for line in $lines; do
    if [[ $line == 'HEADER'$'\t'* ]]; then
      header=${line#*$'\t'}
      continue
    fi
    insert=${line%%$'\t'*}
    display=${line#*$'\t'}
    [[ $display == $insert ]] && display=''
    pairs+="$insert:${display}"
  done

  # Tag is dotcmds; lets us scope styles even more tightly if wanted
  _describe -t dotcmds "$header" pairs
}

compdef _dot dot
