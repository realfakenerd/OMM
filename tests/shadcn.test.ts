import { describe, it, expect } from 'vitest';

describe('shadcn-svelte dependencies', () => {
    it('should have clsx installed', async () => {
        try {
            await import('clsx');
            expect(true).toBe(true);
        } catch (e) {
            expect(e).toBeUndefined(); // This will fail if not installed
        }
    });

    it('should have tailwind-merge installed', async () => {
        try {
            await import('tailwind-merge');
            expect(true).toBe(true);
        } catch (e) {
            expect(e).toBeUndefined(); // This will fail if not installed
        }
    });
});
