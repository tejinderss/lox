import { expect, test } from 'vitest';
import { main } from './index';

test('main greets world', () => {
    expect(main()).toBe("Hello, World!")
})

test('main greets Satinder', () => {
    expect(main("Satinder")).toBe("Hello, Satinder!")
})