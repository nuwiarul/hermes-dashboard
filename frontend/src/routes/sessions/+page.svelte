<script lang="ts">
    import SessionCard from '$lib/components/SessionCard.svelte';
    import PullToRefresh from '$lib/shared/components/PullToRefresh.svelte';
    import { onMount } from 'svelte';
    
    let sessions = $state<any[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);
    let search = $state('');
    let sortBy = $state<'newest' | 'oldest' | 'messages'>('newest');
    let refreshing = $state(false);
    
    let filteredSessions = $derived(
        sessions
            .filter(s => 
                search === '' || 
                s.title?.toLowerCase().includes(search.toLowerCase()) ||
                s.id.toLowerCase().includes(search.toLowerCase()) ||
                s.source?.toLowerCase().includes(search.toLowerCase())
            )
            .sort((a, b) => {
                if (sortBy === 'newest') return (b.started_at || 0) - (a.started_at || 0);
                if (sortBy === 'oldest') return (a.started_at || 0) - (b.started_at || 0);
                return (b.message_count || 0) - (a.message_count || 0);
            })
    );

    let totalSessions = $derived(sessions.length);
    let activeSessions = $derived(sessions.filter(s => !s.ended_at).length);
    let totalMessages = $derived(sessions.reduce((sum, s) => sum + (s.message_count || 0), 0));
    
    async function fetchSessions() {
        try {
            const baseUrl = import.meta.env.VITE_API_BASE_URL;
            const res = await fetch(`${baseUrl}/api/sessions`, { credentials: 'include' });
            if (!res.ok) throw new Error('Failed to fetch sessions');
            const data = await res.json();
            sessions = data.sessions;
        } catch (e) {
            console.error('Failed to fetch sessions:', e);
            error = 'Failed to load sessions';
        } finally {
            loading = false;
            refreshing = false;
        }
    }
    
    async function handleRefresh() {
        refreshing = true;
        await fetchSessions();
    }
    
    onMount(fetchSessions);
</script>

<div class="max-w-7xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">Sessions</h1>
        <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Browse and manage your AI agent sessions</p>
    </div>

    {#if loading}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
            {#each Array(6) as _}
                <div class="bg-white dark:bg-gray-800 rounded-xl p-4 sm:p-5 animate-pulse border border-gray-100 dark:border-gray-700">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="h-5 w-5 sm:h-6 sm:w-6 bg-gray-200 dark:bg-gray-700 rounded"></div>
                        <div class="h-4 sm:h-5 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
                    </div>
                    <div class="space-y-2">
                        <div class="h-3 sm:h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                        <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
                    </div>
                </div>
            {/each}
        </div>
    {:else if error}
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4 sm:p-6 text-sm sm:text-base text-red-700 dark:text-red-400">
            {error}
            <button onclick={handleRefresh} class="ml-3 px-3 py-1.5 bg-red-100 dark:bg-red-800/30 hover:bg-red-200 dark:hover:bg-red-700/30 rounded-lg text-sm font-medium min-h-[44px]">Retry</button>
        </div>
    {:else}
        <div class="grid grid-cols-2 gap-2 sm:gap-3 md:gap-4 mb-4 sm:mb-6">
            <div class="bg-white dark:bg-gray-800 rounded-xl p-3 sm:p-4 shadow-sm border border-gray-100 dark:border-gray-700">
                <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Total Sessions</p>
                <p class="text-lg sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{totalSessions}</p>
            </div>
            <div class="bg-white dark:bg-gray-800 rounded-xl p-3 sm:p-4 shadow-sm border border-gray-100 dark:border-gray-700">
                <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Active</p>
                <p class="text-lg sm:text-2xl font-bold text-green-600 dark:text-green-400">{activeSessions}</p>
            </div>
            <div class="bg-white dark:bg-gray-800 rounded-xl p-3 sm:p-4 shadow-sm border border-gray-100 dark:border-gray-700">
                <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Total Messages</p>
                <p class="text-lg sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{totalMessages.toLocaleString()}</p>
            </div>
            <div class="bg-white dark:bg-gray-800 rounded-xl p-3 sm:p-4 shadow-sm border border-gray-100 dark:border-gray-700">
                <p class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Filtered</p>
                <p class="text-lg sm:text-2xl font-bold text-blue-600 dark:text-blue-400">{filteredSessions.length}</p>
            </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl p-3 sm:p-4 shadow-sm mb-4 sm:mb-6 border border-gray-100 dark:border-gray-700">
            <div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
                <div class="flex-1">
                    <div class="relative">
                        <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500">🔍</span>
                        <input 
                            type="text" 
                            placeholder="Search sessions..."
                            bind:value={search}
                            class="w-full pl-10 pr-4 py-3 text-sm border border-gray-200 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent min-h-[44px] bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500"
                        />
                    </div>
                </div>
                
                <div class="flex items-center gap-2">
                    <span class="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Sort:</span>
                    <select 
                        bind:value={sortBy}
                        class="px-3 py-3 text-sm border border-gray-200 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 min-h-[44px] text-gray-900 dark:text-gray-100"
                    >
                        <option value="newest">Newest</option>
                        <option value="oldest">Oldest</option>
                        <option value="messages">Most Messages</option>
                    </select>
                </div>
            </div>
        </div>

        {#if filteredSessions.length === 0}
            <div class="bg-white dark:bg-gray-800 rounded-xl p-8 sm:p-12 text-center shadow-sm border border-gray-100 dark:border-gray-700">
                <span class="text-3xl sm:text-4xl mb-3 sm:mb-4 block">🔍</span>
                <p class="text-gray-500 dark:text-gray-400 text-base sm:text-lg">No sessions found</p>
                <p class="text-gray-400 dark:text-gray-500 text-xs sm:text-sm mt-1">Try adjusting your search or filters</p>
                <button onclick={handleRefresh} class="mt-4 px-4 py-2.5 bg-blue-500 text-white rounded-lg hover:bg-blue-600 text-sm font-medium min-h-[44px]">Refresh</button>
            </div>
        {:else}
            <PullToRefresh onrefresh={handleRefresh} {refreshing}>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
                    {#each filteredSessions as session (session.id)}
                        <SessionCard {session} />
                    {/each}
                </div>
            </PullToRefresh>
        {/if}
    {/if}
</div>
