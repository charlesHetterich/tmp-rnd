import { Command } from "commander";
import { program } from "./command-structure";

const isHidden = (c: Command) => Boolean((c as any)._hidden);
const isFlag = (s: string) => s.startsWith("-");
const starts = (s: string, frag: string) => (frag ? s.startsWith(frag) : true);

type Candidate = { insert: string; display?: string };
type CompletionResult = {
    items: Candidate[];
    note?: string;
    headerColor?: string;
};

function completion(toks: string[], frag: string): CompletionResult {
    let cmd = program;
    const parts = toks[0] === program.name() ? toks.slice(1) : toks;

    const walk = frag === "" ? parts : parts.slice(0, -1);
    for (const t of walk) {
        if (t === "" || isFlag(t)) continue;
        const sub = cmd.commands.find(
            (c) => c.name() === t || c.aliases().includes(t)
        );
        if (sub) cmd = sub;
        else {
            return {
                items: [],
                note: `Error: '${t}' is not a subcommand of '${
                    cmd.name() || program.name()
                }'.`,
                headerColor: "red",
            };
        }
    }

    const prev = parts.at(-1);

    // value-completion
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
            return {
                items: choices
                    .filter((v) => v.startsWith(needle))
                    .map((v) => ({ insert: v })),
                headerColor: "green",
            };
        }
    }

    // flag-completion
    if (frag.startsWith("-")) {
        const flags = cmd.options.flatMap(
            (o) => [o.short, o.long].filter(Boolean) as string[]
        );
        return {
            items: flags
                .filter((f) => f.startsWith(frag))
                .map((f) => ({ insert: f })),
            headerColor: "green",
        };
    }

    // NEW: mid-word invalid token → emit an error (no space needed)
    if (frag !== "" && !frag.startsWith("-")) {
        const anyMatch = cmd.commands.some(
            (c) =>
                c.name().startsWith(frag) ||
                c.aliases().some((a) => a.startsWith(frag))
        );
        if (!anyMatch) {
            const where = cmd.name() || program.name();
            return {
                items: [],
                note: `Error: '${frag}' is not a subcommand of '${where}'.`,
                headerColor: "red",
            };
        }
    }

    // subcommand-completion
    return {
        items: cmd.commands
            .filter((c) => !isHidden(c))
            .filter(
                (c) =>
                    c.name().startsWith(frag) ||
                    c.aliases().some((a) => a.startsWith(frag))
            )
            .map((c) => ({
                insert: c.name(),
                display: `-- ${c.description()}`,
            })),
        headerColor: "green",
    };
}

function printCompletion(
    items: Candidate[],
    note?: string,
    headerColor?: string
) {
    if (headerColor) process.stdout.write(`COLOR\t${headerColor}\n`);
    if (note) process.stdout.write(`NOTE\t${note}\n`);

    process.stdout.write(`HEADER\t dot commands\n`);
    const max = items.reduce((m, it) => Math.max(m, it.insert.length), 0);
    for (const it of items) {
        const desc = it.display ?? "";
        const aligned = `${it.insert.padEnd(max + 2)}${desc}`;
        process.stdout.write(`${it.insert}\x1F${aligned}\n`);
    }
}

export function handleComplete(argv: string[]) {
    const curEq = argv.find((a) => a.startsWith("--cur="));
    const curIdx = argv.indexOf("--cur");
    const cur = curEq
        ? curEq.slice("--cur=".length)
        : curIdx >= 0
        ? argv[curIdx + 1] ?? ""
        : "";

    const dd = argv.indexOf("--");
    const toks = dd >= 0 ? argv.slice(dd + 1) : argv.slice(1);

    const { items, note, headerColor } = completion(toks, cur);
    printCompletion(items, note, headerColor);
}
