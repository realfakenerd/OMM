import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import Page from '../src/routes/+page.svelte';
import { render, fireEvent, cleanup, waitFor } from '@testing-library/svelte';
import * as saf from '../src-tauri/plugins/saf/guest-js/index.ts';

// Mock the saf module
vi.mock('../src-tauri/plugins/saf/guest-js/index.ts', () => ({
    checkPermission: vi.fn(),
    requestPermission: vi.fn(),
    readFile: vi.fn(),
    writeFile: vi.fn(),
    listDir: vi.fn(),
}));

describe('Page Component Storage Persistence', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        localStorage.clear();
    });

    afterEach(() => {
        cleanup();
    });

    it('should read from localStorage on mount', async () => {
        const storedUri = 'content://com.openmw/config';
        localStorage.setItem('openmw_config_uri', storedUri);
        (saf.checkPermission as any).mockResolvedValue(true);
        (saf.readFile as any).mockResolvedValue(''); // mock empty config

        render(Page);

        await waitFor(() => {
             // Check if it reached the state where permission is checked
             expect(saf.checkPermission).toHaveBeenCalledWith(storedUri);
        });
    });

    it('should save to localStorage when permission is granted', async () => {
        (saf.requestPermission as any).mockResolvedValue('content://new/uri');
        (saf.readFile as any).mockResolvedValue('');

        const { getByText } = render(Page);

        // Wait for initial render and check (no permission initially)
        await waitFor(() => {
             expect(getByText('Grant Permission')).toBeDefined();
        });

        const grantButton = getByText('Grant Permission');
        await fireEvent.click(grantButton);

        await waitFor(() => {
            expect(localStorage.getItem('openmw_config_uri')).toBe('content://new/uri');
        });
    });
});
