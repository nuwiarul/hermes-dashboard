<script lang="ts">
    import StatsCard from '$lib/components/StatsCard.svelte';
    import { stats, status } from '$lib/stores/status';
    import { onMount } from 'svelte';
    
    let loading = $state(true);
    
    onMount(async () => {
        setTimeout(() => { loading = false; }, 1000);
    });

    function formatNumber(n: number): string {
        if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
        if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
        return n.toLocaleString();
    }
</script>

<div class="max-w-7xl mx-auto">
    <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">Dashboard</h1>
    <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400 mb-6 sm:mb-8">Monitor & Control your AI Agent</p>
    
    {#if loading}
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 md:gap-6">
            {#each Array(4) as _}
                <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-6 animate-pulse border border-gray-100 dark:border-gray-700">
                    <div class="h-6 w-6 sm:h-8 sm:w-8 bg-gray-200 dark:bg-gray-700 rounded mb-3 sm:mb-4"></div>
                    <div class="h-3 sm:h-4 bg-gray-200 dark:bg-gray-700 rounded w-20 sm:w-24 mb-2"></div>
                    <div class="h-5 sm:h-8 bg-gray-200 dark:bg-gray-700 rounded w-12 sm:w-16"></div>
                </div>
            {/each}
        </div>
    {:else}
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 md:gap-6">
            <StatsCard title="Total Sessions" value={formatNumber($stats.total_sessions)} icon="💬" trend="up" />
            <StatsCard title="Total Messages" value={formatNumber($stats.total_messages)} icon="📨" trend="up" />
            <StatsCard title="Tool Calls" value={formatNumber($stats.total_tool_calls || 0)} icon="🔧" />
            <StatsCard title="Estimated Cost" value={`$${($stats.estimated_cost_usd || 0).toFixed(2)}`} icon="💰" />
        </div>
        
        <div class="mt-6 sm:mt-8 grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-6 border border-gray-100 dark:border-gray-700">
                <h3 class="text-base sm:text-lg font-semibold mb-3 sm:mb-4 text-gray-900 dark:text-gray-100">Today's Activity</h3>
                <div class="space-y-3 sm:space-y-4">
                    <div class="flex items-center justify-between">
                        <span class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Sessions Today</span>
                        <span class="text-xl sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{$stats.sessions_today || 0}</span>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Messages Today</span>
                        <span class="text-xl sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{$stats.messages_today || 0}</span>
                    </div>
                </div>
            </div>
            
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-6 border border-gray-100 dark:border-gray-700">
                <h3 class="text-base sm:text-lg font-semibold mb-3 sm:mb-4 text-gray-900 dark:text-gray-100">Active Sources</h3>
                {#if !$stats.active_sources || $stats.active_sources.length === 0}
                    <p class="text-sm sm:text-base text-gray-500 dark:text-gray-400">No active sources</p>
                {:else}
                    <div class="space-y-2 sm:space-y-3">
                        {#each $stats.active_sources as source}
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <span class="text-lg sm:text-xl">
                                        {#if source.source === 'telegram'}📱
                                        {:else if source.source === 'discord'}🎮
                                        {:else if source.source === 'cli'}💻
                                        {:else}🔗
                                        {/if}
                                    </span>
                                    <span class="text-sm sm:text-base capitalize text-gray-700 dark:text-gray-300">{source.source}</span>
                                </div>
                                <span class="bg-gray-100 dark:bg-gray-700 px-2 sm:px-3 py-1 rounded-full text-xs sm:text-sm font-medium text-gray-700 dark:text-gray-300">
                                    {source.count} sessions
                                </span>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>
        
        <div class="mt-6 sm:mt-8 bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-6 border border-gray-100 dark:border-gray-700">
            <h3 class="text-base sm:text-lg font-semibold mb-3 sm:mb-4 text-gray-900 dark:text-gray-100">Token Usage</h3>
            <div class="grid grid-cols-2 gap-2 sm:gap-4">
                <div class="text-center p-3 sm:p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Input Tokens</p>
                    <p class="text-base sm:text-xl font-bold text-gray-900 dark:text-gray-100">{formatNumber($stats.total_input_tokens || 0)}</p>
                </div>
                <div class="text-center p-3 sm:p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Output Tokens</p>
                    <p class="text-base sm:text-xl font-bold text-gray-900 dark:text-gray-100">{formatNumber($stats.total_output_tokens || 0)}</p>
                </div>
                <div class="text-center p-3 sm:p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Cache Read</p>
                    <p class="text-base sm:text-xl font-bold text-gray-900 dark:text-gray-100">{formatNumber($stats.total_cache_read_tokens || 0)}</p>
                </div>
                <div class="text-center p-3 sm:p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
                    <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Tool Calls</p>
                    <p class="text-base sm:text-xl font-bold text-gray-900 dark:text-gray-100">{formatNumber($stats.total_tool_calls || 0)}</p>
                </div>
            </div>
        </div>
        
        <div class="mt-4 sm:mt-6 text-center text-xs sm:text-sm text-gray-400 dark:text-gray-500">
            {#if $status.online}
                <span class="text-green-500">●</span> Real-time updates active (WebSocket)
            {:else}
                <span class="text-red-500">●</span> WebSocket disconnected — auto-reconnecting...
            {/if}
        </div>
    {/if}
</div>
