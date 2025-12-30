import { describe, it, expect, vi, afterEach } from 'vitest';
import Button from '../src/lib/components/ui/button/button.svelte';
import { render, fireEvent, cleanup } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';

describe('Button Component', () => {
    afterEach(() => {
        cleanup();
    });

    it('should render correctly', () => {
        const snippet = createRawSnippet(() => ({
            render: () => 'Click me'
        }));
        const { getByRole } = render(Button, { props: { children: snippet } });
        const button = getByRole('button');
        expect(button).toBeDefined();
        expect(button.textContent).toBe('Click me');
    });

    it('should handle clicks', async () => {
        const handleClick = vi.fn();
        const snippet = createRawSnippet(() => ({
            render: () => 'Click me'
        }));
        const { getByRole } = render(Button, { props: { children: snippet, onclick: handleClick } });
        const button = getByRole('button');
        
        await fireEvent.click(button);
        expect(handleClick).toHaveBeenCalled();
    });
});
