import { expect, test } from 'vitest';
import { main } from './index';

test('main returns', () => {
    expect(main()).toBe('Hello, World!')
})