# dot zsh completion (sourceable, non-invasive)

if ! typeset -p _comps &>/dev/null; then
  autoload -Uz compinit && compinit -i
fi
zmodload -i zsh/complist 2>/dev/null

# Only affect `dot`
zstyle ':completion:*:*:dot:*' list-colors '=(#b)(account)(*)=34'
zstyle ':completion:*:*:dot:*' group-name ''
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
      # Ask your CLI; each line:  INSERT<US>DISPLAY  (US = \x1F)
      lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}") || return 1
      (( $#lines )) || return 1

      # Parse protocol; don't use the reserved name "line" here
      local row insert display
      local SEP=$'\x1F'
      for row in $lines; do
        if [[ $row == 'HEADER'$'\t'* ]]; then
          header=${row#*$'\t'}
        elif [[ $row == 'NOTE'$'\t'* ]]; then
          notes+=("${row#*$'\t'}")
        elif [[ $row == 'FILES'$'\t'* ]]; then
          want_files=${row#*$'\t'}   # 'DIRS' or 'FILES'
        else
          insert=${row%%${SEP}*}
          display=${row#*${SEP}}
          [[ $display == $insert ]] && display=''
          items+="$insert"
          descs+="$display"
        fi
      done

      # Hand matches to zsh; force one-per-line and keep header/group
      if (( $#items )); then
        _wanted dotcmds expl "$header" \
          compadd -1 -d descs -- $items && ret=0
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
