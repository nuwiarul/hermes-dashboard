export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://api-hermes.vinrul.my.id';

export async function apiFetch<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${url}`, {
        ...options,
        credentials: 'include', // CRITICAL for cross-origin cookies
        headers: {
            'Content-Type': 'application/json',
            ...options?.headers,
        },
    });

    if (!response.ok) {
        throw new Error(`API error: ${response.status} ${response.statusText}`);
    }

    return response.json();
}
