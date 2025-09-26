import { Command } from "commander";
import { program } from "./command-structure";

type Candidate = { insert: string; display?: string };

const isHidden = (c: Command) => Boolean((c as any)._hidden);

// tiny helper
const starts = (s: string, frag: string) => (frag ? s.startsWith(frag) : true);

function completion(toks: string[], frag: string): Candidate[] {
    let cmd = program;
    const parts = toks[0] === program.name() ? toks.slice(1) : toks;

    // ✅ If frag is empty (fish after-space), include the last token in the walk.
    const walk = frag === "" ? parts : parts.slice(0, -1);
    for (const t of walk) {
        const sub = cmd.commands.find(
            (c) => !isHidden(c) && (c.name() === t || c.aliases().includes(t))
        );
        if (sub) cmd = sub;
    }

    const prev = parts.at(-1);

    // value for an option (prev flag needs a value, or --opt=<frag>)
    if (
        (prev?.startsWith("-") && !prev.includes("=")) ||
        (frag.startsWith("--") && frag.includes("="))
    ) {
        const flag = (frag.includes("=") ? frag.split("=")[0] : prev) ?? "";
        const opt = cmd.options.find(
            (o) => o.long === flag || o.short === flag
        );
        const choices = (opt && (opt as any).argChoices) as
            | string[]
            | undefined;
        const needsValue =
            !!opt && ((opt as any).required || (opt as any).optional);
        if (needsValue && choices) {
            const needle = frag.split("=").pop() || "";
            return choices
                .filter((v) => starts(v, needle))
                .map((v) => ({ insert: v }));
        }
    }

    // completing flags
    if (frag.startsWith("-")) {
        const flags = cmd.options.flatMap(
            (o) => [o.short, o.long].filter(Boolean) as string[]
        );
        return flags.filter((f) => starts(f, frag)).map((f) => ({ insert: f }));
    }

    // completing subcommands
    return cmd.commands
        .filter((c) => !isHidden(c))
        .filter(
            (c) =>
                starts(c.name(), frag) ||
                c.aliases().some((a) => starts(a, frag))
        )
        .map((c) => ({
            insert: c.name(),
            // allow literal tabs in display text for alignment
            display: `-- ${c.description()}`,
        }));
}

function printCompletion(items: Candidate[]) {
    process.stdout.write(`HEADER\t dot commands\n`);
    const max = items.reduce((m, it) => Math.max(m, it.insert.length), 0);
    for (const it of items) {
        const desc = it.display ?? "";
        const aligned = `${it.insert.padEnd(max + 2)}${desc}`;
        // INSERT<US>DISPLAY  (US = \x1F separator)
        process.stdout.write(`${it.insert}\x1F${aligned}\n`);
    }
}

export function handleComplete(argv: string[]) {
    // Extract --cur (support --cur foo and --cur=foo)
    const curEq = argv.find((a) => a.startsWith("--cur="));
    const curIdx = argv.indexOf("--cur");
    const cur = curEq
        ? curEq.slice("--cur=".length)
        : curIdx >= 0
        ? argv[curIdx + 1] ?? ""
        : "";

    // Everything after `--` is the token vector from zsh
    const dd = argv.indexOf("--");
    const toks = dd >= 0 ? argv.slice(dd + 1) : argv.slice(1);

    const items = completion(toks, cur);
    printCompletion(items);
}
