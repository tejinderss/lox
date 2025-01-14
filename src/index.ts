import { parseArgs } from 'node:util';
import { run_prompt, run_file } from './lox.ts';

export function main(): void {
    const { positionals } = parseArgs({
        allowPositionals: true,
    });
    if (positionals.length > 1) {
        throw new Error("Usage: lox [script]");
    } else if (positionals.length == 1) {
        run_file(positionals[0]);
    } else {
        run_prompt();
    }
}

main();