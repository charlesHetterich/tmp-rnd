# ~/.config/fish/completions/dot.fish
function __dot_complete
    dot __complete fish -- (commandline -opc)  # tokens
end
complete -c dot -a "(__dot_complete)"
