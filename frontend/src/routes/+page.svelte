<script lang="ts">
    import StatsCard from '$lib/components/StatsCard.svelte';
    import { onMount } from 'svelte';
    
    let stats = $state({
        total_sessions: 0,
        total_messages: 0,
        sessions_today: 0,
        messages_today: 0,
        active_sources: [] as { source: string; count: number }[],
        total_tool_calls: 0,
        estimated_cost_usd: 0,
    });
    
    let loading = $state(true);
    let error = $state<string | null>(null);
    
    onMount(async () => {
        try {
            const baseUrl = import.meta.env.VITE_API_BASE_URL;
            const res = await fetch(`${baseUrl}/api/stats`);
            if (!res.ok) throw new Error('Failed to fetch stats');
            stats = await res.json();
        } catch (e) {
            console.error('Failed to fetch stats:', e);
            error = 'Failed to load stats';
        } finally {
            loading = false;
        }
    });

    function formatNumber(n: number): string {
        if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
        if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
        return n.toLocaleString();
    }
</script>

<div class="max-w-7xl mx-auto">
    <h1 class="text-3xl font-bold text-gray-900 mb-2">Dashboard</h1>
    <p class="text-gray-600 mb-8">Welcome to Hermes Dashboard — Monitor & Control your AI Agent</p>
    
    {#if loading}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {#each Array(4) as _}
                <div class="bg-white rounded-xl shadow-sm p-6 animate-pulse">
                    <div class="h-8 w-8 bg-gray-200 rounded mb-4"></div>
                    <div class="h-4 bg-gray-200 rounded w-24 mb-2"></div>
                    <div class="h-8 bg-gray-200 rounded w-16"></div>
                </div>
            {/each}
        </div>
    {:else if error}
        <div class="bg-red-50 border border-red-200 rounded-xl p-6 text-red-700">
            {error}
        </div>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <StatsCard 
                title="Total Sessions" 
                value={formatNumber(stats.total_sessions)} 
                icon="💬" 
                trend="up"
            />
            <StatsCard 
                title="Total Messages" 
                value={formatNumber(stats.total_messages)} 
                icon="📨" 
                trend="up"
            />
            <StatsCard 
                title="Tool Calls" 
                value={formatNumber(stats.total_tool_calls)} 
                icon="🔧"
            />
            <StatsCard 
                title="Estimated Cost" 
                value={`$${stats.estimated_cost_usd.toFixed(2)}`} 
                icon="💰"
            />
        </div>
        
        <div class="mt-8 grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- Sessions Today -->
            <div class="bg-white rounded-xl shadow-sm p-6">
                <h3 class="text-lg font-semibold mb-4">Today's Activity</h3>
                <div class="space-y-4">
                    <div class="flex items-center justify-between">
                        <span class="text-gray-600">Sessions Today</span>
                        <span class="text-2xl font-bold text-gray-900">{stats.sessions_today}</span>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-gray-600">Messages Today</span>
                        <span class="text-2xl font-bold text-gray-900">{stats.messages_today}</span>
                    </div>
                </div>
            </div>
            
            <!-- Active Sources -->
            <div class="bg-white rounded-xl shadow-sm p-6">
                <h3 class="text-lg font-semibold mb-4">Active Sources</h3>
                {#if stats.active_sources.length === 0}
                    <p class="text-gray-500">No active sources</p>
                {:else}
                    <div class="space-y-3">
                        {#each stats.active_sources as source}
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <span class="text-xl">
                                        {#if source.source === 'telegram'}📱
                                        {:else if source.source === 'discord'}🎮
                                        {:else if source.source === 'cli'}💻
                                        {:else}🔗
                                        {/if}
                                    </span>
                                    <span class="capitalize">{source.source}</span>
                                </div>
                                <span class="bg-gray-100 px-3 py-1 rounded-full text-sm font-medium">
                                    {source.count} sessions
                                </span>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>
        
        <!-- Token Usage -->
        <div class="mt-8 bg-white rounded-xl shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-4">Token Usage</h3>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div class="text-center p-4 bg-gray-50 rounded-lg">
                    <p class="text-sm text-gray-500">Input Tokens</p>
                    <p class="text-xl font-bold text-gray-900">{formatNumber(stats.total_input_tokens)}</p>
                </div>
                <div class="text-center p-4 bg-gray-50 rounded-lg">
                    <p class="text-sm text-gray-500">Output Tokens</p>
                    <p class="text-xl font-bold text-gray-900">{formatNumber(stats.total_output_tokens)}</p>
                </div>
                <div class="text-center p-4 bg-gray-50 rounded-lg">
                    <p class="text-sm text-gray-500">Cache Read</p>
                    <p class="text-xl font-bold text-gray-900">{formatNumber(stats.total_cache_read_tokens)}</p>
                </div>
                <div class="text-center p-4 bg-gray-50 rounded-lg">
                    <p class="text-sm text-gray-500">Tool Calls</p>
                    <p class="text-xl font-bold text-gray-900">{formatNumber(stats.total_tool_calls)}</p>
                </div>
            </div>
        </div>
    {/if}
</div>
