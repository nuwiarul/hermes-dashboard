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
    <!-- Header -->
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-1 sm:mb-2">Sessions</h1>
        <p class="text-sm sm:text-base text-gray-600">Browse and manage your AI agent sessions</p>
    </div>

    {#if loading}
        <!-- Loading Skeleton -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
            {#each Array(6) as _}
                <div class="bg-white rounded-xl p-4 sm:p-5 animate-pulse">
                    <div class="flex items-center gap-2 mb-3">
                        <div class="h-5 w-5 sm:h-6 sm:w-6 bg-gray-200 rounded"></div>
                        <div class="h-4 sm:h-5 bg-gray-200 rounded w-3/4"></div>
                    </div>
                    <div class="space-y-2">
                        <div class="h-3 sm:h-4 bg-gray-200 rounded w-1/2"></div>
                        <div class="h-3 bg-gray-200 rounded w-1/3"></div>
                    </div>
                </div>
            {/each}
        </div>
    {:else if error}
        <!-- Error State -->
        <div class="bg-red-50 border border-red-200 rounded-xl p-4 sm:p-6 text-sm sm:text-base text-red-700">
            {error}
            <button 
                onclick={handleRefresh}
                class="ml-3 px-3 py-1.5 bg-red-100 hover:bg-red-200 rounded-lg text-sm font-medium min-h-[44px]"
            >
                Retry
            </button>
        </div>
    {:else}
        <!-- Stats Bar -->
        <div class="grid grid-cols-2 gap-2 sm:gap-3 md:gap-4 mb-4 sm:mb-6">
            <div class="bg-white rounded-xl p-3 sm:p-4 shadow-sm">
                <p class="text-xs sm:text-sm text-gray-500">Total Sessions</p>
                <p class="text-lg sm:text-2xl font-bold text-gray-900">{totalSessions}</p>
            </div>
            <div class="bg-white rounded-xl p-3 sm:p-4 shadow-sm">
                <p class="text-xs sm:text-sm text-gray-500">Active</p>
                <p class="text-lg sm:text-2xl font-bold text-green-600">{activeSessions}</p>
            </div>
            <div class="bg-white rounded-xl p-3 sm:p-4 shadow-sm">
                <p class="text-xs sm:text-sm text-gray-500">Total Messages</p>
                <p class="text-lg sm:text-2xl font-bold text-gray-900">{totalMessages.toLocaleString()}</p>
            </div>
            <div class="bg-white rounded-xl p-3 sm:p-4 shadow-sm">
                <p class="text-xs sm:text-sm text-gray-500">Filtered</p>
                <p class="text-lg sm:text-2xl font-bold text-blue-600">{filteredSessions.length}</p>
            </div>
        </div>

        <!-- Search & Filter Bar -->
        <div class="bg-white rounded-xl p-3 sm:p-4 shadow-sm mb-4 sm:mb-6">
            <div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
                <!-- Search -->
                <div class="flex-1">
                    <div class="relative">
                        <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">🔍</span>
                        <input 
                            type="text" 
                            placeholder="Search sessions..."
                            bind:value={search}
                            class="w-full pl-10 pr-4 py-3 text-sm border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent min-h-[44px]"
                        />
                    </div>
                </div>
                
                <!-- Sort -->
                <div class="flex items-center gap-2">
                    <span class="text-xs sm:text-sm text-gray-500">Sort:</span>
                    <select 
                        bind:value={sortBy}
                        class="px-3 py-3 text-sm border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white min-h-[44px]"
                    >
                        <option value="newest">Newest</option>
                        <option value="oldest">Oldest</option>
                        <option value="messages">Most Messages</option>
                    </select>
                </div>
            </div>
        </div>

        <!-- Sessions List with Pull to Refresh -->
        {#if filteredSessions.length === 0}
            <div class="bg-white rounded-xl p-8 sm:p-12 text-center shadow-sm">
                <span class="text-3xl sm:text-4xl mb-3 sm:mb-4 block">🔍</span>
                <p class="text-gray-500 text-base sm:text-lg">No sessions found</p>
                <p class="text-gray-400 text-xs sm:text-sm mt-1">Try adjusting your search or filters</p>
                <button 
                    onclick={handleRefresh}
                    class="mt-4 px-4 py-2.5 bg-blue-500 text-white rounded-lg hover:bg-blue-600 text-sm font-medium min-h-[44px]"
                >
                    Refresh
                </button>
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
