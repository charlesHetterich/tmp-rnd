type Candidate = { insert: string; display?: string };

const COMMANDS: Candidate[] = [
    { insert: "fetch", display: "Fetch a thias fdasdfasdfasfasdfng" },
    { insert: "fetcher", display: "Fetch aNOTHER tdfasdfasd asdf asfhing" },
    { insert: "fender", display: "you know the besa fdasfasdfasdfasdfs dnder" },
    {
        insert: "init",
        display: "Initialize pro asfasdfasdfasdf asd fasd d fdd ject",
    },
    { insert: "list", display: "List thingsfd dfd df d" },
    { insert: "no wayl..", display: "random" },
];

function compute(tokens: string[], cur: string): Candidate[] {
    if (tokens[0] === "dot") tokens = tokens.slice(1);
    const p = cur ?? "";
    return COMMANDS.filter((c) => c.insert.startsWith(p));
}

export function completion(argv: string[]) {
    const shell = argv[3] || "zsh";
    const curIdx = argv.indexOf("--cur");
    let cur = curIdx >= 0 ? argv[curIdx + 1] || "" : "";

    const tokStart = argv.indexOf("--");
    const toks = tokStart >= 0 ? argv.slice(tokStart + 1) : [];

    // if a numeric index ever leaks in, map to fragment
    if (/^\d+$/.test(cur)) {
        const i = parseInt(cur, 10) - 1;
        cur = toks[i] ?? cur;
    }

    const items = compute(toks, cur);

    if (shell === "zsh") {
        // header first

        process.stdout.write(`HEADER\t${toks}|${cur}\n`);
        // each item: <insert>\t<display>
        for (const it of items) {
            const display = it.display ?? it.insert;
            process.stdout.write(`${it.insert}\t${it.insert}:${display}\n`);
        }
    } else {
        // fallback: plain names
        for (const it of items) process.stdout.write(`${it.insert}\n`);
    }
}
