<script lang="ts">
    import { apiFetch } from '$lib/shared/utils/api';
    import { onMount } from 'svelte';
    
    interface ToolsetInfo {
        name: string;
        description: string;
        enabled: boolean;
        category: string;
    }
    
    let toolsets = $state<ToolsetInfo[]>([]);
    let loading = $state(true);
    let toggling = $state<string | null>(null);
    let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);
    
    let grouped = $derived(() => {
        const groups: Record<string, ToolsetInfo[]> = {};
        for (const t of toolsets) {
            if (!groups[t.category]) groups[t.category] = [];
            groups[t.category].push(t);
        }
        return groups;
    });
    
    let enabledCount = $derived(toolsets.filter(t => t.enabled).length);
    let disabledCount = $derived(toolsets.filter(t => !t.enabled).length);
    
    onMount(async () => { await loadToolsets(); });
    
    async function loadToolsets() {
        try {
            const data = await apiFetch<{ toolsets: ToolsetInfo[]; disabled_count: number }>('/api/tools/toolsets');
            toolsets = data.toolsets;
        } catch (e) {
            console.error('Failed to load toolsets:', e);
            message = { type: 'error', text: 'Failed to load toolsets' };
        } finally { loading = false; }
    }
    
    async function toggleToolset(name: string, enabled: boolean) {
        toggling = name; message = null;
        try {
            const data = await apiFetch<{ success: boolean; message: string }>('/api/tools/toggle-toolset', {
                method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name, enabled }),
            });
            if (data.success) {
                message = { type: 'success', text: data.message };
                toolsets = toolsets.map(t => t.name === name ? { ...t, enabled } : t);
            } else { message = { type: 'error', text: data.message }; }
        } catch (e) {
            console.error('Failed to toggle toolset:', e);
            message = { type: 'error', text: 'Failed to toggle toolset' };
        } finally { toggling = null; }
    }
</script>

<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <span class="text-2xl">🛠️</span>
            <div>
                <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">Tool Manager</h2>
                <p class="text-sm text-gray-500 dark:text-gray-400">Enable or disable agent tools</p>
            </div>
        </div>
        <div class="flex items-center gap-3 text-sm">
            <span class="text-green-600 dark:text-green-400 font-medium">{enabledCount} enabled</span>
            <span class="text-gray-400 dark:text-gray-500">·</span>
            <span class="text-red-600 dark:text-red-400 font-medium">{disabledCount} disabled</span>
        </div>
    </div>
    
    {#if loading}
        <div class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
    {:else}
        {#if message}
            <div class="mb-4 p-3 rounded-lg {message.type === 'success' ? 'bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-300' : 'bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-300'}">{message.text}</div>
        {/if}
        
        {#each Object.entries(grouped()) as [category, items]}
            <div class="mb-6">
                <h3 class="text-sm font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-3">{category}</h3>
                <div class="space-y-2">
                    {#each items as toolset}
                        <div class="flex items-center justify-between p-3 rounded-lg border {toolset.enabled ? 'border-gray-200 dark:border-gray-600 bg-white dark:bg-gray-700/50' : 'border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20'}">
                            <div class="flex-1">
                                <div class="flex items-center gap-2">
                                    <span class="font-medium text-gray-900 dark:text-gray-100">{toolset.name}</span>
                                    {#if !toolset.enabled}
                                        <span class="px-2 py-0.5 text-xs font-medium bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-300 rounded">Disabled</span>
                                    {/if}
                                </div>
                                <p class="text-sm text-gray-500 dark:text-gray-400">{toolset.description}</p>
                            </div>
                            <button
                                onclick={() => toggleToolset(toolset.name, !toolset.enabled)}
                                disabled={toggling === toolset.name}
                                class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {toolset.enabled ? 'bg-green-600' : 'bg-gray-300 dark:bg-gray-600'} disabled:opacity-50"
                            >
                                <span class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {toolset.enabled ? 'translate-x-6' : 'translate-x-1'}"></span>
                            </button>
                        </div>
                    {/each}
                </div>
            </div>
        {/each}
    {/if}
</div>
