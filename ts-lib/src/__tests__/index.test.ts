import { describe, it, expect } from 'vitest';
import { greet } from '../engine';

describe('greet()', () => {
    it('should return greeting message', () => {
        expect(greet('Duc')).toBe('Hello, Duc from TypeScript!');
    });
});
