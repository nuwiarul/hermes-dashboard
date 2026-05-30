export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://api-hermes.vinrul.my.id';

export async function apiFetch<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${url}`, {
        ...options,
        credentials: 'include', // Always send cookies
    });

    // If 401, check auth and redirect to login
    if (response.status === 401) {
        // Don't call checkAuth here to avoid loop — just redirect
        window.location.href = '/login';
        throw new Error('Unauthorized');
    }

    if (!response.ok) {
        throw new Error(`API error: ${response.status} ${response.statusText}`);
    }

    return response.json();
}
