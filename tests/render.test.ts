import { describe, it, expect } from 'vitest';
import Button from '../src/lib/components/ui/button/button.svelte';
import { render } from '@testing-library/svelte';
import { createRawSnippet } from 'svelte';

describe('Button Component', () => {
    it('should render correctly', () => {
        const snippet = createRawSnippet(() => ({
            render: () => 'Click me'
        }));
        const { getByRole } = render(Button, { props: { children: snippet } });
        const button = getByRole('button');
        expect(button).toBeDefined();
        expect(button.textContent).toBe('Click me');
    });
});
