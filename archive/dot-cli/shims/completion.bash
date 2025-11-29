# dot bash completion (sourceable)

_dot_bash() {
  local cur prev words cword
  # Bash completion API vars
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"

  # Ask your CLI for candidates.
  # Protocol:
  #   HEADER\t<text>
  #   NOTE\t<text>
  #   FILES\tFILES|DIRS
  #   <insert>\x1F<display>
  local -a lines items
  local want_files=""
  local IFS=$'\n'

  # Build argv vector to send after `--` (bash's COMP_WORDS already has program name at [0])
  # shellcheck disable=SC2206
  local -a send_words=( "${COMP_WORDS[@]}" )

  # Capture output; silence errors
  mapfile -t lines < <(dot __complete bash --cur "$cur" -- "${send_words[@]}" 2>/dev/null) || return 1
  ((${#lines[@]})) || return 1

  local line insert display
  local US=$'\x1F' TAB=$'\t'

  for line in "${lines[@]}"; do
    if [[ $line == "HEADER"$TAB* ]]; then
      # not used in bash UI
      :
    elif [[ $line == "NOTE"$TAB* ]]; then
      :
    elif [[ $line == "FILES"$TAB* ]]; then
      want_files=${line#*$'\t'}   # "FILES" or "DIRS"
    else
      # Split on Unit Separator (US) to get insert/display
      insert=${line%%"$US"*}
      display=${line#*"$US"}  # unused in bash
      items+=("$insert")
    fi
  done

  # If CLI returned items, use them.
  if ((${#items[@]})); then
    COMPREPLY=()
    # Use compgen for proper filtering against $cur
    COMPREPLY+=( $(compgen -W "$(printf '%s\n' "${items[@]}")" -- "$cur") )
    # If these are commands/flags, usually you *do* want a space after selection.
    # If you don’t, uncomment the next line:
    # compopt -o nospace 2>/dev/null
    return 0
  fi

  # Otherwise, honor FILES/DIRS hint
  if [[ $want_files == "DIRS" ]]; then
    compopt -o filenames 2>/dev/null
    COMPREPLY=( $(compgen -d -- "$cur") )
    return 0
  elif [[ $want_files == "FILES" ]]; then
    compopt -o filenames 2>/dev/null
    COMPREPLY=( $(compgen -f -- "$cur") )
    return 0
  fi

  return 1
}

# Bind to `dot`
complete -o default -F _dot_bash dot
