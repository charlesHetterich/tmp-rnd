// cli-spec.ts
import { Command, Option } from "commander";
import { initFoundry, initHardHat, initInk } from "./commands";

const fqName = (cmd: Command) => {
    const parts: string[] = [];
    for (
        let c: Command | null = cmd;
        c && c.name();
        c = c.parent as Command | null
    ) {
        parts.unshift(c.name());
    }
    return parts.join(" ");
};

// Generic printer: (...args, opts, cmd)
const printInvocation =
    (label?: string) =>
    (...args: any[]) => {
        const cmd = args.at(-1) as Command;
        const opts =
            args.length >= 2 && typeof args.at(-2) === "object"
                ? args.at(-2)
                : {};
        const positionals = args.slice(0, -2);
        const out = {
            command: fqName(cmd),
            label,
            args: positionals,
            opts,
        };
        // Pretty, stable keys
        console.log(JSON.stringify(out, null, 2));
    };

export const program = new Command("dot")
    .description(
        "Polkadot toolbox — unified, wide-thin wrapper over core ecosystem tools"
    )
    .version("0.1.0")
    .showHelpAfterError()
    .enablePositionalOptions()
    .passThroughOptions();

// ---------- list ----------
const fmtOpt = new Option("-f, --format <format>")
    .choices(["json", "csv", "human"] as const)
    .default("human");

const list = new Command("list").description(
    "Discover templates, tutorials, devcontainers, and dynamic sources"
);

list.command("templates")
    .description("List available project templates")
    .addOption(fmtOpt)
    .action(printInvocation("list templates"));

list.command("tutorials")
    .description("List curated tutorials")
    .addOption(fmtOpt)
    .action(printInvocation("list tutorials"));

list.command("devcontainers")
    .description("List first-party devcontainers")
    .addOption(fmtOpt)
    .action(printInvocation("list devcontainers"));

list.command("accounts")
    .description("List locally known accounts (or from a managed registry)")
    .addOption(fmtOpt)
    .action(printInvocation("list accounts"));

program.addCommand(list);

// ---------- init ----------
const init = new Command("init").description(
    "Initialize a project from a template and set up its environment"
);

const initCommonFlags = [
    new Option("--env-only", "Only set up environment (skip template files)"),
    new Option("--template-only", "Only fetch template files (skip env setup)"),
];

const initContracts = init
    .command("contracts")
    .description("Initialize a smart-contracts project")
    .addOption(initCommonFlags[0])
    .addOption(initCommonFlags[1]);

initContracts
    .command("hardhat")
    .description("Initialize a Hardhat smart-contracts project")
    .argument("<dir>", "target project directory")
    .action(initHardHat);

initContracts
    .command("foundry")
    .description("Initialize a Foundry smart-contracts project")
    .argument("<dir>", "target project directory")
    .action(initFoundry);

initContracts
    .command("ink")
    .description("Initialize an ink! smart-contracts project")
    .argument("<dir>", "target project directory")
    .action(initInk);

init.command("dapp")
    .description("Initialize a dapp project")
    .argument("<dir>", "target project directory")
    .addOption(initCommonFlags[0])
    .addOption(initCommonFlags[1])
    .action(printInvocation("init dapp"));

init.command("pallet")
    .description("Initialize a pallet project")
    .argument("<dir>", "target project directory")
    .addOption(initCommonFlags[0])
    .addOption(initCommonFlags[1])
    .action(printInvocation("init pallet"));

program.addCommand(init);

// ---------- fetch ----------
const fetch = new Command("fetch").description(
    "Fetch content by id into a destination directory (wide wrapper over curated sources)"
);

fetch
    .command("template")
    .description("Fetch a template by id")
    .argument("<id>", "template id")
    .argument("<dir>", "destination directory")
    .action(printInvocation("fetch template"));

fetch
    .command("binary")
    .description("Fetch a binary by id (e.g., subkey, nodes)")
    .argument("<id>", "binary id")
    .argument("<dir>", "destination directory")
    .action(printInvocation("fetch binary"));

program.addCommand(fetch);

// ---------- account ----------
const account = new Command("account")
    .description(
        "Keypair utilities and on-chain lookups (wraps subkey + RPC where useful)"
    )
    .enablePositionalOptions(); // <-- add this

account
    .command("generate")
    .description("Generate a new keypair (delegates to subkey under the hood)")
    .allowUnknownOption(true)
    .passThroughOptions()
    .action(printInvocation("account generate"));

account
    .command("info")
    .description("Show metadata for an account")
    .argument("<id>", "account/address/alias")
    .action(printInvocation("account info"));

account
    .command("balance")
    .description("Show on-chain balance for an account")
    .argument("<id>", "account/address/alias")
    .action(printInvocation("account balance"));

account
    .command("configure")
    .description(
        "Configure a locally known account (labels, defaults, network)"
    )
    .argument("<id>", "account/address/alias")
    .action(printInvocation("account configure"));

program.addCommand(account);
