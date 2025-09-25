#!/usr/bin/env node

import chalk from "chalk";

type Candidate = { insert: string; display?: string };

const COMMANDS: Candidate[] = [
    { insert: "fetch", display: "Fetch a thing" },
    { insert: "fetcher", display: "Fetch aNOTHER thing" },
    { insert: "fender", display: "you know the bender" },
    { insert: "init", display: "Initialize project" },
    { insert: "list", display: "List things" },
    { insert: "no wayl..", display: "random" },
];

function compute(tokens: string[], cur: string): Candidate[] {
    if (tokens[0] === "dot") tokens = tokens.slice(1);
    const p = cur ?? "";
    return COMMANDS.filter((c) => c.insert.startsWith(p));
}

function tokensFromArgv(argv: string[]): string[] {
    const i = argv.indexOf("--");
    return i >= 0 ? argv.slice(i + 1) : [];
}

function main() {
    const argv = process.argv;

    if (argv[2] === "__complete") {
        const shell = argv[3] || "zsh";
        const curIdx = argv.indexOf("--cur");
        let cur = curIdx >= 0 ? argv[curIdx + 1] || "" : "";
        const toks = tokensFromArgv(argv);

        // if a numeric index ever leaks in, map to fragment
        if (/^\d+$/.test(cur)) {
            const i = parseInt(cur, 10) - 1;
            cur = toks[i] ?? cur;
        }

        const items = compute(toks, cur);

        if (shell === "zsh") {
            // header first
            process.stdout.write(`HEADER\t${chalk.green("dot commands")}\n`);
            // each item: <insert>\t<display>
            for (const it of items) {
                const display = it.display ?? it.insert;
                process.stdout.write(`${it.insert}\t${display}\n`);
            }
        } else {
            // fallback: plain names
            for (const it of items) process.stdout.write(`${it.insert}\n`);
        }
        return;
    }

    // normal handlers
    const cmd = argv[2];
    switch (cmd) {
        case "list":
            console.log("listing…");
            break;
        case "fetch":
            console.log("fetching…");
            break;
        case "init":
            console.log("initializing…");
            break;
        default:
            console.log("dot <list|fetch|init>");
    }
}

main();
