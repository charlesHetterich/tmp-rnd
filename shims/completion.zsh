# dot zsh completion (dynamic header color + single-line note)

if ! typeset -p _comps &>/dev/null; then
  autoload -Uz compinit && compinit -i
fi
zmodload -i zsh/complist 2>/dev/null

_dot_pd() {
  emulate -L zsh
  setopt typeset_silent
  local curcontext="$curcontext" state
  local -a lines items descs notes
  local header='dot commands' want_files='' color=''
  local curfrag=${words[$CURRENT]}
  local ret=1

  _arguments -C '*: :->seg'

  case $state in
    seg)
      lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}") || return 1
      (( $#lines )) || return 1

      local row insert display
      local SEP=$'\x1F'
      for row in $lines; do
        if [[ $row == 'COLOR'$'\t'* ]]; then
          color=${row#*$'\t'}
        elif [[ $row == 'HEADER'$'\t'* ]]; then
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

      # Dynamic header color just for this invocation
      local oldfmt
      zstyle -s ':completion:*:*:dot:*:descriptions' format oldfmt
      local newfmt='%F{green}-- %d --%f'
      [[ -n $color ]] && newfmt="%F{$color}-- %d --%f"
      zstyle ':completion:*:*:dot:*:descriptions' format "$newfmt"

      if (( $#items )); then
        _wanted dotcmds expl "$header" \
          compadd -1 -d descs -- $items && ret=0
      fi

      if (( ! $#items && $#notes )); then
        _message "$notes[1]"
        ret=1
      fi

      if [[ $want_files == DIRS ]]; then
        _files -/ && ret=0
      elif [[ $want_files == FILES ]]; then
        _files && ret=0
      fi

      # Restore style
      if [[ -n $oldfmt ]]; then
        zstyle ':completion:*:*:dot:*:descriptions' format "$oldfmt"
      else
        zstyle -d ':completion:*:*:dot:*:descriptions' format
      fi

      return $ret
    ;;
  esac
}

compdef -d dot
compdef _dot_pd dot
