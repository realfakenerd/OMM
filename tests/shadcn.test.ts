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

    it('should have components.json configuration file', async () => {
        const fs = await import('fs/promises');
        const path = await import('path');
        const configPath = path.resolve(process.cwd(), 'components.json');
        
        try {
            await fs.access(configPath);
            expect(true).toBe(true);
        } catch {
            expect(true).toBe(false); // Should fail if file doesn't exist
        }
    });

    it('should correctly merge tailwind classes using cn utility', async () => {
        const { cn } = await import('../src/lib/utils');
        const result = cn('bg-red-500', 'bg-blue-500');
        expect(result).toBe('bg-blue-500'); // tailwind-merge should resolve this
    });
});
