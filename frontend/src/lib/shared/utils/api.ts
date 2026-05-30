export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://api-hermes.vinrul.my.id';

let isRefreshing = false;
let refreshPromise: Promise<boolean> | null = null;

async function tryRefresh(): Promise<boolean> {
    try {
        const res = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
            method: 'POST',
            credentials: 'include',
        });
        return res.ok;
    } catch {
        return false;
    }
}

export async function apiFetch<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${url}`, {
        ...options,
        credentials: 'include',
    });

    // If 401, try refresh token before giving up
    if (response.status === 401) {
        // Don't try refresh for auth endpoints (login, logout, refresh itself)
        if (url.startsWith('/api/auth/')) {
            window.location.href = '/login';
            throw new Error('Unauthorized');
        }

        // Use shared refresh promise to avoid multiple refresh calls
        if (!isRefreshing) {
            isRefreshing = true;
            refreshPromise = tryRefresh();
        }

        const refreshSuccess = await refreshPromise;
        isRefreshing = false;
        refreshPromise = null;

        if (refreshSuccess) {
            // Refresh succeeded — retry original request
            const retryResponse = await fetch(`${API_BASE_URL}${url}`, {
                ...options,
                credentials: 'include',
            });

            if (!retryResponse.ok) {
                throw new Error(`API error: ${retryResponse.status} ${retryResponse.statusText}`);
            }

            return retryResponse.json();
        } else {
            // Refresh failed — redirect to login
            window.location.href = '/login';
            throw new Error('Unauthorized');
        }
    }

    if (!response.ok) {
        throw new Error(`API error: ${response.status} ${response.statusText}`);
    }

    return response.json();
}
