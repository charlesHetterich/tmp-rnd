#!/usr/bin/env node
type Candidate = { text: string; desc?: string };

const COMMANDS: Candidate[] = [
    { text: "list", desc: "List things" },
    { text: "fetch", desc: "Fetch a thing" },
    { text: "init", desc: "Initialize project" },
    { text: "WHAT", desc: "random" },
];

function compute(tokens: string[], cur: string): Candidate[] {
    // Remove leading executable if present
    if (tokens[0] === "dot") tokens = tokens.slice(1);

    // Only root-level completion for now
    const prefix = cur ?? "";
    return COMMANDS.filter((c) => c.text.startsWith(prefix));
}

function isCompleteMode(argv: string[]): boolean {
    return argv[2] === "__complete";
}

function printForShell(shell: string, items: Candidate[]) {
    const withDesc = shell === "zsh" || shell === "fish";
    for (const it of items) {
        console.log(`${it.text}\t${it.desc}\n`);
    }
}

function tokensFromArgv(argv: string[]): string[] {
    const sep = argv.indexOf("--");
    return sep >= 0 ? argv.slice(sep + 1) : [];
}

function main() {
    const argv = process.argv;

    if (isCompleteMode(argv)) {
        const shell = argv[3] || "zsh";
        // Example call shapes:
        //   dot __complete bash --cur <frag> -- <tokens...>
        const curIdx = argv.indexOf("--cur");
        const cur = curIdx >= 0 ? argv[curIdx + 1] || "" : "";
        const toks = tokensFromArgv(argv);
        const items = compute(toks, cur);
        printForShell(shell, items);
        return;
    }

    // Dummy handlers for the three commands
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
