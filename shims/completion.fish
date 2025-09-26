# dot fish completion (dynamic; uses your __complete endpoint)
function __dot_candidates --description 'dynamic candidates for dot'
    set -l cur (commandline -ct)        # current fragment
    set -l words (commandline -o)       # tokenized argv
    set -l US \x1f

    set -l out (dot __complete fish --cur "$cur" -- $words 2>/dev/null)

    for line in $out
        # 1) Ignore metadata lines (robustly; tab not required for the match)
        if string match -rq '^HEADER' -- $line
            continue
        else if string match -rq '^NOTE' -- $line
            continue
        else if string match -rq '^FILES' -- $line
            # Let fish fall back to file/dir completion if you want; we ignore here.
            continue
        end

        # 2) Only process rows that contain the US separator
        if not string match -q "*$US*" -- $line
            continue
        end

        set -l parts (string split -m1 $US -- $line)
        set -l insert  $parts[1]
        set -l display $parts[2]

        if test -z "$display"
            echo $insert
        else
            # Fish shows a description when there's a literal TAB in the candidate
            printf '%s\t%s\n' $insert $display
        end
    end
end

# Disable default file completion (-f) when our function provides items.
# (Keeps fish from adding random files/dirs alongside your command list.)
complete -c dot -f -a "(__dot_candidates)"
