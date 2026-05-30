<script lang="ts">
    import { auth } from '$lib/stores/auth';
    import { theme } from '$lib/stores/theme';
    import { haptic } from '$lib/shared/utils/touch';

    let { status = 'online', model = 'Unknown' }: { 
        status?: 'online' | 'offline'; 
        model?: string 
    } = $props();

    async function handleLogout() {
        haptic(15);
        await auth.logout();
    }

    function handleThemeToggle() {
        haptic(5);
        theme.toggle();
    }

    let themeIcon = $derived($theme === 'dark' ? '🌙' : $theme === 'light' ? '☀️' : '💻');
    let themeLabel = $derived($theme === 'dark' ? 'Dark' : $theme === 'light' ? 'Light' : 'System');
</script>

<header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-3 sm:px-4 md:px-6 py-3 sm:py-4 flex justify-between items-center gap-2 transition-colors">
    <div class="flex items-center gap-2 min-w-0">
        <span class="text-xs sm:text-sm text-gray-500 dark:text-gray-400 hidden sm:inline">Model:</span>
        <span class="font-mono text-xs sm:text-sm text-gray-900 dark:text-gray-100 truncate">{model}</span>
    </div>
    
    <div class="flex items-center gap-2 sm:gap-3 shrink-0">
        <div class="flex items-center gap-1.5 sm:gap-2">
            <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-green-500' : 'bg-red-500'}"></span>
            <span class="text-xs sm:text-sm text-gray-700 dark:text-gray-300 capitalize hidden sm:inline">{status}</span>
        </div>
        
        {#if $auth.username}
            <span class="text-xs sm:text-sm text-gray-500 dark:text-gray-400 hidden md:inline">{$auth.username}</span>
        {/if}
        
        <!-- Theme toggle -->
        <button
            onclick={handleThemeToggle}
            class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 active:bg-gray-200 dark:active:bg-gray-600 transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center"
            title="Theme: {themeLabel}"
            aria-label="Toggle theme"
        >
            <span class="text-lg">{themeIcon}</span>
        </button>
        
        <button
            onclick={handleLogout}
            class="text-xs sm:text-sm text-gray-500 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition cursor-pointer px-3 py-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 active:bg-red-100 dark:active:bg-red-900/30 min-h-[44px] flex items-center"
        >
            Logout
        </button>
    </div>
</header>
