import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://api-hermes.vinrul.my.id';

interface AuthState {
  isAuthenticated: boolean;
  username: string | null;
  isLoading: boolean;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    isAuthenticated: false,
    username: null,
    isLoading: true,
  });

  return {
    subscribe,

    /// Check login status by calling /api/auth/me (validates cookie)
    /// If access_token expired, try refresh first
    checkAuth: async () => {
      if (!browser) return;
      update(s => ({ ...s, isLoading: true }));

      try {
        const res = await fetch(`${API_BASE_URL}/api/auth/me`, {
          credentials: 'include',
        });

        if (res.ok) {
          const data = await res.json();
          set({
            isAuthenticated: true,
            username: data.username || null,
            isLoading: false,
          });
          return;
        }

        // Access token invalid — try refresh
        const refreshRes = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
          method: 'POST',
          credentials: 'include',
        });

        if (refreshRes.ok) {
          // Refresh succeeded — retry checkAuth
          const retryRes = await fetch(`${API_BASE_URL}/api/auth/me`, {
            credentials: 'include',
          });

          if (retryRes.ok) {
            const data = await retryRes.json();
            set({
              isAuthenticated: true,
              username: data.username || null,
              isLoading: false,
            });
            return;
          }
        }

        // Both failed — not authenticated
        set({
          isAuthenticated: false,
          username: null,
          isLoading: false,
        });
      } catch {
        set({
          isAuthenticated: false,
          username: null,
          isLoading: false,
        });
      }
    },

    /// Login: call /api/auth/login, server sets HttpOnly cookies
    login: async (username: string, password: string): Promise<{ success: boolean; message: string }> => {
      try {
        const res = await fetch(`${API_BASE_URL}/api/auth/login`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({ username, password }),
        });

        const data = await res.json();

        if (!res.ok) {
          return { success: false, message: data.message || 'Login failed' };
        }

        return { success: true, message: 'Login successful' };
      } catch {
        return { success: false, message: 'Failed to connect to server' };
      }
    },

    /// Logout: call /api/auth/logout to clear cookies
    logout: async () => {
      try {
        await fetch(`${API_BASE_URL}/api/auth/logout`, {
          method: 'POST',
          credentials: 'include',
        });
      } catch {
        // Ignore errors, clear state anyway
      }

      set({
        isAuthenticated: false,
        username: null,
        isLoading: false,
      });

      if (browser) {
        window.location.href = '/login';
      }
    },
  };
}

export const auth = createAuthStore();
