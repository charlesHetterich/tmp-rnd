# ~/.config/dot/completion.bash
_dot_complete() {
  local cur="${COMP_WORDS[COMP_CWORD]}"
  local IFS=$'\n'
  COMPREPLY=($(dot __complete bash --cur "$cur" -- "${COMP_WORDS[@]}"))
}
complete -o default -o bashdefault -F _dot_complete dot
