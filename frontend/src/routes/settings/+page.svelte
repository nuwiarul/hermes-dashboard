<script lang="ts">
    import { onMount } from 'svelte';
    
    let config = $state<any>(null);
    let loading = $state(true);
    let error = $state<string | null>(null);
    
    onMount(async () => {
        try {
            const baseUrl = import.meta.env.VITE_API_BASE_URL;
            const res = await fetch(`${baseUrl}/api/config`, { credentials: 'include' });
            if (!res.ok) throw new Error('Failed to fetch config');
            config = await res.json();
        } catch (e) {
            console.error('Failed to fetch config:', e);
            error = 'Failed to load configuration';
        } finally {
            loading = false;
        }
    });

    function getStatusColor(enabled: boolean): string {
        return enabled ? 'text-green-600 bg-green-50' : 'text-gray-600 bg-gray-50';
    }
</script>

<div class="max-w-4xl mx-auto">
    <!-- Header -->
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-1 sm:mb-2">Settings</h1>
        <p class="text-sm sm:text-base text-gray-600">View system configuration and information</p>
    </div>

    {#if loading}
        <!-- Loading Skeleton -->
        <div class="space-y-4 sm:space-y-6">
            {#each Array(3) as _}
                <div class="bg-white rounded-xl p-4 sm:p-6 animate-pulse">
                    <div class="h-5 sm:h-6 bg-gray-200 rounded w-1/4 mb-3 sm:mb-4"></div>
                    <div class="space-y-2 sm:space-y-3">
                        <div class="h-3 sm:h-4 bg-gray-200 rounded w-full"></div>
                        <div class="h-3 sm:h-4 bg-gray-200 rounded w-3/4"></div>
                        <div class="h-3 sm:h-4 bg-gray-200 rounded w-1/2"></div>
                    </div>
                </div>
            {/each}
        </div>
    {:else if error}
        <!-- Error State -->
        <div class="bg-red-50 border border-red-200 rounded-xl p-4 sm:p-6 text-sm sm:text-base text-red-700">
            {error}
        </div>
    {:else}
        <!-- System Status -->
        <div class="bg-white rounded-xl shadow-sm p-4 sm:p-6 mb-4 sm:mb-6">
            <h2 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 flex items-center gap-2">
                <span>🖥️</span> System Status
            </h2>
            <div class="grid grid-cols-2 gap-2 sm:gap-3 md:gap-4">
                <div class="text-center p-3 sm:p-4 bg-gray-50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500">Config Version</p>
                    <p class="text-lg sm:text-2xl font-bold text-gray-900">{config?.config_version || 'Unknown'}</p>
                </div>
                <div class="text-center p-3 sm:p-4 bg-gray-50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500">Default Model</p>
                    <p class="text-lg sm:text-2xl font-bold text-gray-900 truncate">{config?.model || 'Unknown'}</p>
                </div>
                <div class="text-center p-3 sm:p-4 bg-gray-50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500">Provider</p>
                    <p class="text-lg sm:text-2xl font-bold text-gray-900 capitalize">{config?.provider || 'Unknown'}</p>
                </div>
                <div class="text-center p-3 sm:p-4 {getStatusColor(config?.gateway_enabled)} rounded-lg">
                    <p class="text-xs sm:text-sm">Gateway</p>
                    <p class="text-lg sm:text-2xl font-bold">{config?.gateway_enabled ? 'Enabled' : 'Disabled'}</p>
                </div>
            </div>
        </div>

        <!-- Model Configuration -->
        <div class="bg-white rounded-xl shadow-sm p-4 sm:p-6 mb-4 sm:mb-6">
            <h2 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 flex items-center gap-2">
                <span>🤖</span> Model Configuration
            </h2>
            <div class="grid grid-cols-1 gap-0">
                <div class="flex justify-between py-2 sm:py-3 border-b border-gray-100">
                    <span class="text-sm sm:text-base text-gray-600">Default Model</span>
                    <span class="font-mono text-xs sm:text-sm text-gray-900 truncate ml-4">{config?.model || 'Unknown'}</span>
                </div>
                <div class="flex justify-between py-2 sm:py-3 border-b border-gray-100">
                    <span class="text-sm sm:text-base text-gray-600">Provider</span>
                    <span class="font-mono text-xs sm:text-sm text-gray-900 capitalize">{config?.provider || 'Unknown'}</span>
                </div>
                <div class="flex justify-between py-2 sm:py-3 border-b border-gray-100">
                    <span class="text-sm sm:text-base text-gray-600">Max Turns</span>
                    <span class="font-mono text-xs sm:text-sm text-gray-900">{config?.max_turns || 'Unknown'}</span>
                </div>
                <div class="flex justify-between py-2 sm:py-3">
                    <span class="text-sm sm:text-base text-gray-600">Gateway Status</span>
                    <span class="text-xs sm:text-sm font-medium {config?.gateway_enabled ? 'text-green-600' : 'text-gray-500'}">
                        {config?.gateway_enabled ? '✓ Active' : '○ Inactive'}
                    </span>
                </div>
            </div>
        </div>

        <!-- About -->
        <div class="bg-white rounded-xl shadow-sm p-4 sm:p-6">
            <h2 class="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 flex items-center gap-2">
                <span>ℹ️</span> About
            </h2>
            <div class="space-y-2 sm:space-y-3 text-sm sm:text-base text-gray-600">
                <p><strong>Hermes Dashboard</strong> — Monitor & Control your AI Agent</p>
                <p>
                    Built with <span class="font-semibold text-gray-900">Rust (Axum)</span> backend and 
                    <span class="font-semibold text-gray-900">SvelteKit</span> frontend.
                </p>
                <p class="text-xs sm:text-sm text-gray-400">
                    Version 0.1.0 • 
                    <a href="https://github.com/nuwiarul/hermes-dashboard" target="_blank" rel="noopener" class="text-blue-500 hover:underline">
                        View on GitHub
                    </a>
                </p>
            </div>
        </div>
    {/if}
</div>
