import { Command, InvalidArgumentError, Option } from "commander";

const parseIntStrict = (v: string) => {
    const n = Number.parseInt(v, 10);
    if (Number.isNaN(n)) throw new InvalidArgumentError("Expected integer");
    return n;
};

const program = new Command()
    .name("dot")
    .description("Polkadot toolbox")
    .version("0.1.0");

program
    .command("fetch")
    .description("Fetch something")
    .requiredOption(
        "-n, --network <name>",
        "target network",
        process.env.NETWORK
    )
    .option("-r, --retries <n>", "retry count", parseIntStrict, 3)
    .action(async (opts) => {
        // no `this` nonsense—opts + args are explicit
        // await fetchTask(opts.network, opts.retries);
    });

const admin = new Command("admin").description("Admin ops");
admin
    .command("user <id>")
    .description("Manage a user")
    .addOption(
        new Option("--format <format>")
            .choices(["text", "json"])
            .default("text")
    )
    .option("--enable", "enable user")
    .option("--role <role>", "set role")
    .action((id, opts) => {
        console.log("admin user", id, opts);
    });

program.addCommand(admin);

// Pass-through to subtools: `dot run -- <anything>`
program
    .command("run")
    .allowExcessArguments(true)
    .allowUnknownOption(true)
    .description("Run underlying tool with passthrough")
    .action((_opts, cmd) => {
        const argvAfterDoubleDash = cmd.parent!.args.slice(
            cmd.parent!.args.indexOf("run") + 1
        );
        runPassthrough(argvAfterDoubleDash);
    });

await program.parseAsync(process.argv);
