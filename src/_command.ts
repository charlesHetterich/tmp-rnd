enum ArgumentType {
    Boolean,
    String,
    Number,
}
type ArgSpec = ArgumentType | string[];

export type OptSpec = {
    name: string; // e.g. "--network <name>"
    desc: string;
    arg?: string[];
};

export type Ctx = {
    argv: string[]; // raw argv after '--'
    words: string[]; // zsh $words slice you pass in
    cur: string; // fragment being completed (or last, if you forced it)
    index: number; // 0-based index in words/argv you’re completing
};
export type __CmdSpec = {
    subCommands?: __CmdSpec[];
    process: (remainingArgs: string[]) => Promise<void> | void;
};

export function CmdSpec<T extends __CmdSpec>(spec: __CmdSpec) {}

const cmd2: __CmdSpec = {
    subCommands: [],
    process: function (args: string[]) {
        this.subCommands;
    },
};
CmdSpec({
    subCommands: [],
    process: (args: string[]) => {},
});

// export type _CmdSpec = {
//     name: string; // "fetch"
//     summary: string;
//     desc?: string;
//     args?: string; // e.g. "<resource> [extra...]"
//     options?: OptSpec[];
//     run: (ctx: Ctx) => Promise<void> | void;
//     complete?: (ctx: Ctx) => Promise<Candidate[]> | Candidate[];
// };

type Command = {
    name: string;
    description: string;
    action: () => void;
};

// class Commander {
//     private argv: string[];
// }
