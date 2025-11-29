import { program } from "./command-structure";
import { handleComplete } from "./completion-handler";

async function main() {
    const argv = process.argv;

    // Intercept for autocompletion command
    if (argv[2] === "__complete") {
        handleComplete(argv);
        process.exit(0);
    }

    // Regular CLI command
    await program.parseAsync(argv);
}

main();
