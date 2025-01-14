export function main(name?: string): string {
    return `Hello, ${name ?? 'World'}!`;
}

console.log(main());