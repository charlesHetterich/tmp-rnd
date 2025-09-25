# dot zsh completion (sourceable, non-invasive)

if ! typeset -p _comps &>/dev/null; then
  autoload -Uz compinit && compinit -i
fi
zmodload -i zsh/complist 2>/dev/null

# Only affect `dot`
zstyle ':completion:*:*:dot:*' group-name ''
zstyle ':completion:*:*:dot:*' menu select=2
zstyle ':completion:*:*:dot:*' autolist always
zstyle ':completion:*:*:dot:*' list-ambiguous yes
zstyle ':completion:*:*:dot:*' list-rows-first 'no'
zstyle ':completion:*:*:dot:*' list-packed 'no'
zstyle ':completion:*:*:dot:*:descriptions' format '%F{green}-- %d --%f'

# NOTE: Graphviz ships a _dot completer; avoid collisions.
_dot_pd() {
  emulate -L zsh
  local curcontext="$curcontext" state
  local -a lines items descs notes
  local header='dot commands' want_files=''
  local curfrag=${words[$CURRENT]}
  local ret=1

  # Put zsh into completion "argument" state (bun does this)
  _arguments -C '*: :->seg'

  case $state in
    seg)
      # Ask your CLI; each line:  INSERT<TAB>DISPLAY
      lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}") || return 1
      (( $#lines )) || return 1

      # Parse protocol; don't use the reserved name "line" here
      local row insert display
      for row in $lines; do
        if [[ $row == 'HEADER'$'\t'* ]]; then
          header=${row#*$'\t'}
        elif [[ $row == 'NOTE'$'\t'* ]]; then
          notes+=("${row#*$'\t'}")
        elif [[ $row == 'FILES'$'\t'* ]]; then
          want_files=${row#*$'\t'}   # 'DIRS' or 'FILES'
        else
          insert=${row%%$'\t'*}
          display=${row#*$'\t'}
          [[ $display == $insert ]] && display=''
          items+="$insert"
          descs+="$display"
        fi
      done

      # Non-selectable docs above the list
      local n
      for n in $notes; do
        compadd -X "$n"
        ret=0
      done

      # Single match -> insert immediately
      if (( $#items == 1 )); then
        compadd -U -- "$items[1]" && return 0
      fi

      # Hand matches to zsh in the requested tag; this is the key
      if (( $#items )); then
        local expl
        _wanted dotcmds expl "$header" compadd -d descs -- $items && ret=0
      fi

      # Optional fallback to native file/dir completion
      if [[ $want_files == DIRS ]]; then
        _files -/ && ret=0
      elif [[ $want_files == FILES ]]; then
        _files && ret=0
      fi

      return $ret
    ;;
  esac
}

# Unbind any existing completer for `dot` (e.g., Graphviz), bind ours
compdef -d dot
compdef _dot_pd dot
