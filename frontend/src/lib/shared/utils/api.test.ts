import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock fetch globally
const mockFetch = vi.fn();
vi.stubGlobal('fetch', mockFetch);

describe('apiFetch utility', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        // Set default env
        vi.stubEnv('VITE_API_BASE_URL', 'https://api.example.com');
    });

    it('makes GET request with credentials', async () => {
        mockFetch.mockResolvedValueOnce({
            ok: true,
            json: async () => ({ data: 'test' }),
        });

        const { apiFetch } = await import('./api');
        const result = await apiFetch('/api/test');

        expect(mockFetch).toHaveBeenCalledWith(
            'https://api.example.com/api/test',
            expect.objectContaining({
                credentials: 'include',
            })
        );
        expect(result).toEqual({ data: 'test' });
    });

    it('makes POST request with body', async () => {
        mockFetch.mockResolvedValueOnce({
            ok: true,
            json: async () => ({ success: true }),
        });

        const { apiFetch } = await import('./api');
        await apiFetch('/api/test', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ key: 'value' }),
        });

        expect(mockFetch).toHaveBeenCalledWith(
            'https://api.example.com/api/test',
            expect.objectContaining({
                method: 'POST',
                credentials: 'include',
            })
        );
    });

    it('throws on non-ok response', async () => {
        mockFetch.mockResolvedValueOnce({
            ok: false,
            status: 404,
            json: async () => ({ error: 'Not found' }),
        });

        const { apiFetch } = await import('./api');
        await expect(apiFetch('/api/missing')).rejects.toThrow();
    });
});
