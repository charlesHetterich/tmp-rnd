# dot zsh completions

_dot() {
  local -a lines items notes
  local header='dot commands' want_files='' color=''
  local curfrag=${words[$CURRENT]}

  lines=("${(@f)$(dot __complete zsh --cur "$curfrag" -- "${words[@]}" 2>/dev/null)}") || return 1

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
      items+=("${insert}\:\"${display}\" ")
    fi
  done

  # Dynamic header color just for this invocation
  local newfmt='%F{green}-- %d --%f'
  [[ -n $color ]] && newfmt="%F{$color}-- %d --%f"
  zstyle ':completion:*:*:dot:*:descriptions' format "$newfmt"

  # Print matches/message
  _alternative "args:$header:(($items))"
  if (( $#notes )); then
    _message "$notes[1]"
    ret=1
  fi
}

if ! typeset -p _comps &>/dev/null; then
  autoload -Uz compinit && compinit -i
fi
compdef _dot dot
