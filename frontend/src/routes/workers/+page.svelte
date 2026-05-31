<script lang="ts">
    import { onMount } from 'svelte';
    import { apiFetch } from '$lib/shared/utils/api';

    interface Worker {
        id: number;
        name: string;
        ip: string;
        role: string;
        os: string;
        arch: string;
        ram_total: number;
        disk_total: number;
        capabilities: string;
        status: string;
        current_task: string | null;
        ram_used: number;
        disk_used: number;
        active_model: string | null;
        last_heartbeat: string | null;
        registered_at: string;
    }

    let workers = $state<Worker[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    async function fetchWorkers() {
        try {
            loading = true;
            const data = await apiFetch<{ workers: Worker[]; total: number }>('/api/workers');
            workers = data.workers;
            error = null;
        } catch (e: any) {
            error = e.message || 'Failed to fetch workers';
        } finally {
            loading = false;
        }
    }

    function getStatusColor(status: string): string {
        switch (status) {
            case 'online': return 'bg-green-500';
            case 'busy': return 'bg-yellow-500';
            case 'offline': return 'bg-red-500';
            default: return 'bg-gray-500';
        }
    }

    function formatBytes(mb: number): string {
        if (mb >= 1024) return `${(mb / 1024).toFixed(1)} GB`;
        return `${mb} MB`;
    }

    function formatTime(timeStr: string | null): string {
        if (!timeStr) return 'Never';
        const date = new Date(timeStr + 'Z');
        const now = new Date();
        const diffMs = now.getTime() - date.getTime();
        const diffSec = Math.floor(diffMs / 1000);
        if (diffSec < 60) return `${diffSec}s ago`;
        const diffMin = Math.floor(diffSec / 60);
        if (diffMin < 60) return `${diffMin}m ago`;
        const diffHr = Math.floor(diffMin / 60);
        return `${diffHr}h ago`;
    }

    onMount(() => {
        fetchWorkers();
        // Auto-refresh every 30s
        const interval = setInterval(fetchWorkers, 30000);
        return () => clearInterval(interval);
    });
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">Workers</h1>
        <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Monitor and manage your Hermes instances</p>
    </div>

    {#if loading && workers.length === 0}
        <!-- Skeleton loading -->
        <div class="space-y-4">
            {#each Array(2) as _}
                <div class="bg-white dark:bg-gray-800 rounded-xl p-4 sm:p-6 animate-pulse border border-gray-100 dark:border-gray-700">
                    <div class="flex items-center gap-4">
                        <div class="h-10 w-10 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
                        <div class="flex-1">
                            <div class="h-5 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-2"></div>
                            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    {:else if error}
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4 sm:p-6">
            <p class="text-red-600 dark:text-red-400">⚠️ {error}</p>
            <button onclick={fetchWorkers} class="mt-2 text-sm text-red-600 dark:text-red-400 underline">Retry</button>
        </div>
    {:else if workers.length === 0}
        <div class="bg-white dark:bg-gray-800 rounded-xl p-8 sm:p-12 text-center border border-gray-100 dark:border-gray-700">
            <span class="text-4xl mb-4 block">🖥️</span>
            <p class="text-gray-600 dark:text-gray-400 mb-2">No workers registered yet</p>
            <p class="text-sm text-gray-500 dark:text-gray-500">Workers will appear here when they register via the API</p>
        </div>
    {:else}
        <div class="space-y-4">
            {#each workers as worker (worker.id)}
                <div class="bg-white dark:bg-gray-800 rounded-xl p-4 sm:p-6 border border-gray-100 dark:border-gray-700 hover:shadow-md transition-shadow">
                    <!-- Header -->
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-3">
                            <div class="relative">
                                <span class="text-2xl">
                                    {worker.os === 'windows' ? '🪟' : worker.os === 'linux' ? '🐧' : '💻'}
                                </span>
                                <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-white dark:border-gray-800 {getStatusColor(worker.status)}"></div>
                            </div>
                            <div>
                                <h3 class="font-semibold text-gray-900 dark:text-gray-100">{worker.name}</h3>
                                <p class="text-xs text-gray-500 dark:text-gray-400">{worker.ip} • {worker.role}</p>
                            </div>
                        </div>
                        <div class="text-right">
                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium
                                {worker.status === 'online' ? 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400' :
                                  worker.status === 'busy' ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400' :
                                  'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'}">
                                {worker.status}
                            </span>
                        </div>
                    </div>

                    <!-- Stats -->
                    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-4">
                        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
                            <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">Model</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-gray-100 truncate">{worker.active_model || 'N/A'}</p>
                        </div>
                        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
                            <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">RAM</p>
                            <div class="flex items-center gap-2">
                                <div class="flex-1 bg-gray-200 dark:bg-gray-600 rounded-full h-1.5">
                                    <div class="bg-blue-500 h-1.5 rounded-full" style="width: {worker.ram_total > 0 ? Math.min((worker.ram_used / worker.ram_total) * 100, 100) : 0}%"></div>
                                </div>
                                <span class="text-xs text-gray-600 dark:text-gray-300 whitespace-nowrap">{formatBytes(worker.ram_used)}/{formatBytes(worker.ram_total)}</span>
                            </div>
                        </div>
                        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
                            <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">Disk</p>
                            <div class="flex items-center gap-2">
                                <div class="flex-1 bg-gray-200 dark:bg-gray-600 rounded-full h-1.5">
                                    <div class="bg-purple-500 h-1.5 rounded-full" style="width: {worker.disk_total > 0 ? Math.min((worker.disk_used / worker.disk_total) * 100, 100) : 0}%"></div>
                                </div>
                                <span class="text-xs text-gray-600 dark:text-gray-300 whitespace-nowrap">{formatBytes(worker.disk_used)}/{formatBytes(worker.disk_total)}</span>
                            </div>
                        </div>
                        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
                            <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">Heartbeat</p>
                            <p class="text-sm font-medium text-gray-900 dark:text-gray-100">{formatTime(worker.last_heartbeat)}</p>
                        </div>
                    </div>

                    <!-- Current Task -->
                    {#if worker.current_task}
                        <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-3 mb-3">
                            <p class="text-xs text-blue-600 dark:text-blue-400 mb-1">Current Task</p>
                            <p class="text-sm text-blue-800 dark:text-blue-300">{worker.current_task}</p>
                        </div>
                    {/if}

                    <!-- Capabilities -->
                    {#if worker.capabilities && worker.capabilities !== '[]'}
                        <div class="flex flex-wrap gap-1.5">
                            {#each JSON.parse(worker.capabilities) as cap}
                                <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300">
                                    {cap}
                                </span>
                            {/each}
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}
</div>
