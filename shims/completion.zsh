# dot zsh completion (sourceable)

# Initialize completion (idempotent)
autoload -Uz compinit && compinit -i

# Pretty output like bun's script
zstyle ':completion:*:*:dot:*' group-name ''                  # keep groups separate
zstyle ':completion:*:descriptions' format '%F{green}-- %d --%f'
zstyle ':completion:*' menu select                            # arrow-key selection
# Optional layout tweaks:
# zstyle ':completion:*' list-rows-first 'yes'
# zstyle ':completion:*' list-packed 'no'

# The completion function for `dot`
_dot() {
  emulate -L zsh

  local -a lines items descs
  local curfrag="${words[$CURRENT]}"   # <-- the actual fragment being completed

  # Ask your CLI. It should print "item<TAB>desc" lines for zsh.
  # IMPORTANT: pass tokens as separate args with "${words[@]}"
  lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}")
  (( ${#lines} )) || return 1

  local l
  for l in $lines; do
    if [[ $l == *$'\t'* ]]; then
      items+="${l%%$'\t'*}"
      descs+="${l#*$'\t'}"
    else
      items+="$l"
      descs+=""
    fi
  done

  # One group with a green header
  compadd -J dotcmds -X 'dot commands' -d descs -- $items
  return 0
}

# Bind function to command
compdef _dot dot
