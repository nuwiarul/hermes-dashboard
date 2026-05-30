<script lang="ts">
    import { onMount } from 'svelte';
    
    let jobs = $state<any[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);
    let filter = $state<'all' | 'active' | 'paused'>('all');
    
    let filteredJobs = $derived(
        filter === 'all' ? jobs : jobs.filter(j => j.state === filter)
    );

    let activeCount = $derived(jobs.filter(j => j.state === 'scheduled').length);
    let pausedCount = $derived(jobs.filter(j => j.state === 'paused').length);
    
    onMount(async () => {
        try {
            const baseUrl = import.meta.env.VITE_API_BASE_URL;
            const res = await fetch(`${baseUrl}/api/cron`);
            if (!res.ok) throw new Error('Failed to fetch cron jobs');
            const data = await res.json();
            jobs = data.jobs || [];
        } catch (e) {
            console.error('Failed to fetch cron jobs:', e);
            error = 'Failed to load cron jobs';
        } finally {
            loading = false;
        }
    });

    function getStateColor(state: string): string {
        switch (state) {
            case 'scheduled': return 'bg-green-100 text-green-700';
            case 'paused': return 'bg-yellow-100 text-yellow-700';
            case 'completed': return 'bg-blue-100 text-blue-700';
            default: return 'bg-gray-100 text-gray-700';
        }
    }

    function getStateIcon(state: string): string {
        switch (state) {
            case 'scheduled': return '▶️';
            case 'paused': return '⏸️';
            case 'completed': return '✅';
            default: return '❓';
        }
    }
</script>

<div class="max-w-7xl mx-auto">
    <!-- Header -->
    <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Cron Jobs</h1>
        <p class="text-gray-600">Manage scheduled tasks for your AI agent</p>
    </div>

    {#if loading}
        <!-- Loading Skeleton -->
        <div class="space-y-4">
            {#each Array(3) as _}
                <div class="bg-white rounded-xl p-6 animate-pulse">
                    <div class="flex items-center gap-3 mb-4">
                        <div class="h-6 w-6 bg-gray-200 rounded"></div>
                        <div class="h-5 bg-gray-200 rounded w-1/3"></div>
                        <div class="h-5 bg-gray-200 rounded w-20 ml-auto"></div>
                    </div>
                    <div class="space-y-2">
                        <div class="h-4 bg-gray-200 rounded w-1/2"></div>
                        <div class="h-3 bg-gray-200 rounded w-1/4"></div>
                    </div>
                </div>
            {/each}
        </div>
    {:else if error}
        <!-- Error State -->
        <div class="bg-red-50 border border-red-200 rounded-xl p-6 text-red-700">
            {error}
        </div>
    {:else}
        <!-- Stats Bar -->
        <div class="grid grid-cols-3 gap-4 mb-6">
            <div class="bg-white rounded-xl p-4 shadow-sm">
                <p class="text-sm text-gray-500">Total Jobs</p>
                <p class="text-2xl font-bold text-gray-900">{jobs.length}</p>
            </div>
            <div class="bg-white rounded-xl p-4 shadow-sm">
                <p class="text-sm text-gray-500">Active</p>
                <p class="text-2xl font-bold text-green-600">{activeCount}</p>
            </div>
            <div class="bg-white rounded-xl p-4 shadow-sm">
                <p class="text-sm text-gray-500">Paused</p>
                <p class="text-2xl font-bold text-yellow-600">{pausedCount}</p>
            </div>
        </div>

        <!-- Filter Bar -->
        <div class="bg-white rounded-xl p-4 shadow-sm mb-6">
            <div class="flex items-center gap-2">
                <span class="text-sm text-gray-500">Filter:</span>
                <button 
                    onclick={() => filter = 'all'}
                    class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {filter === 'all' ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
                >
                    All ({jobs.length})
                </button>
                <button 
                    onclick={() => filter = 'active'}
                    class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {filter === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
                >
                    Active ({activeCount})
                </button>
                <button 
                    onclick={() => filter = 'paused'}
                    class="px-3 py-1.5 rounded-lg text-sm font-medium transition-colors {filter === 'paused' ? 'bg-yellow-100 text-yellow-700' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}"
                >
                    Paused ({pausedCount})
                </button>
            </div>
        </div>

        <!-- Jobs List -->
        {#if filteredJobs.length === 0}
            <div class="bg-white rounded-xl p-12 text-center shadow-sm">
                <span class="text-4xl mb-4 block">⏰</span>
                {#if jobs.length === 0}
                    <p class="text-gray-500 text-lg">No cron jobs configured</p>
                    <p class="text-gray-400 text-sm mt-1">Create one using the Hermes agent</p>
                {:else}
                    <p class="text-gray-500 text-lg">No {filter} jobs found</p>
                {/if}
            </div>
        {:else}
            <div class="space-y-4">
                {#each filteredJobs as job (job.id)}
                    <div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition-all border border-gray-100 hover:border-blue-200">
                        <div class="flex items-start justify-between gap-4">
                            <div class="flex-1 min-w-0">
                                <div class="flex items-center gap-3 mb-2">
                                    <span class="text-lg">{getStateIcon(job.state)}</span>
                                    <h3 class="font-semibold text-lg text-gray-900 truncate">
                                        {job.name || 'Unnamed Job'}
                                    </h3>
                                    <span class="text-xs px-2 py-1 rounded-full font-medium {getStateColor(job.state)}">
                                        {job.state}
                                    </span>
                                </div>
                                
                                <div class="flex flex-wrap items-center gap-3 text-sm text-gray-500 mb-3">
                                    <span class="flex items-center gap-1">
                                        📅 {job.schedule}
                                    </span>
                                    {#if job.deliver.length > 0}
                                        <span class="text-gray-300">•</span>
                                        <span class="flex items-center gap-1">
                                            📤 {job.deliver.join(', ')}
                                        </span>
                                    {/if}
                                    {#if job.skills.length > 0}
                                        <span class="text-gray-300">•</span>
                                        <span class="flex items-center gap-1">
                                            🛠️ {job.skills.join(', ')}
                                        </span>
                                    {/if}
                                </div>

                                {#if job.prompt}
                                    <p class="text-sm text-gray-600 bg-gray-50 rounded-lg p-3 font-mono truncate">
                                        {job.prompt}
                                    </p>
                                {/if}

                                <div class="flex flex-wrap items-center gap-4 mt-3 text-xs text-gray-400">
                                    {#if job.next_run}
                                        <span>Next: {job.next_run}</span>
                                    {/if}
                                    {#if job.last_run}
                                        <span>Last: {job.last_run}</span>
                                    {/if}
                                    {#if job.script}
                                        <span>Script: {job.script}</span>
                                    {/if}
                                    {#if job.no_agent}
                                        <span class="bg-gray-100 px-2 py-0.5 rounded">no-agent</span>
                                    {/if}
                                </div>
                            </div>
                        </div>
                        
                        <div class="mt-3 pt-3 border-t border-gray-100">
                            <p class="text-xs text-gray-400 font-mono truncate">ID: {job.id}</p>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    {/if}
</div>
