function main(name?: string): string {
    const name_val = name ?? "World";
    return `Hello, ${name_val}!`;
}

console.log(main());