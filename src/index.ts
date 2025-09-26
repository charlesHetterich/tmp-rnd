import { program } from "./command-structure";
import { handleComplete } from "./completion-handler";

async function main() {
    const argv = process.argv;

    // Bypass Commander entirely for __complete so it never shows up in help
    if (argv[2] === "__complete") {
        handleComplete(argv);
        process.exit(0);
    }

    await program.parseAsync(argv);
}

main();
