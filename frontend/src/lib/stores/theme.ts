/**
 * Theme store — dark/light mode with localStorage persistence
 */
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark' | 'system';

function createThemeStore() {
    const stored = browser ? localStorage.getItem('theme') as Theme : null;
    const { subscribe, set, update } = writable<Theme>(stored || 'system');

    function applyTheme(theme: Theme) {
        if (!browser) return;

        const root = document.documentElement;
        let effectiveTheme: 'light' | 'dark';

        if (theme === 'system') {
            effectiveTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
        } else {
            effectiveTheme = theme;
        }

        root.classList.toggle('dark', effectiveTheme === 'dark');
        localStorage.setItem('theme', theme);
    }

    return {
        subscribe,
        set: (theme: Theme) => {
            set(theme);
            applyTheme(theme);
        },
        toggle: () => {
            update(current => {
                let next: Theme;
                if (current === 'light') next = 'dark';
                else if (current === 'dark') next = 'system';
                else next = 'light';

                applyTheme(next);
                return next;
            });
        },
        init: () => {
            if (!browser) return;
            const stored = localStorage.getItem('theme') as Theme || 'system';
            applyTheme(stored);
            set(stored);

            // Listen for system theme changes
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
                const current = localStorage.getItem('theme') as Theme;
                if (current === 'system') {
                    applyTheme('system');
                }
            });
        }
    };
}

export const theme = createThemeStore();
