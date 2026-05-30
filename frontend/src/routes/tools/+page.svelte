<script lang="ts">
    import { onMount } from 'svelte';
    import type { Component } from 'svelte';
    
    let ModelSwitcher: Component = $state(undefined!);
    let ToolManager: Component = $state(undefined!);
    let SendMessage: Component = $state(undefined!);
    let GatewayControl: Component = $state(undefined!);
    let loaded = $state(false);
    
    onMount(async () => {
        // Lazy load all tool components in parallel
        const [ms, tm, sm, gc] = await Promise.all([
            import('$lib/components/ModelSwitcher.svelte'),
            import('$lib/components/ToolManager.svelte'),
            import('$lib/components/SendMessage.svelte'),
            import('$lib/components/GatewayControl.svelte'),
        ]);
        ModelSwitcher = ms.default;
        ToolManager = tm.default;
        SendMessage = sm.default;
        GatewayControl = gc.default;
        loaded = true;
    });
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">Tools</h1>
        <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Remote control for your AI agent</p>
    </div>
    
    {#if loaded}
        <div class="space-y-4 sm:space-y-6">
            <ModelSwitcher />
            <ToolManager />
            <SendMessage />
            <GatewayControl />
        </div>
    {:else}
        <!-- Skeleton loading for tools -->
        <div class="space-y-4 sm:space-y-6">
            {#each Array(4) as _}
                <div class="bg-white dark:bg-gray-800 rounded-xl p-4 sm:p-6 animate-pulse border border-gray-100 dark:border-gray-700">
                    <div class="h-5 sm:h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/3 mb-3 sm:mb-4"></div>
                    <div class="space-y-2 sm:space-y-3">
                        <div class="h-3 sm:h-4 bg-gray-200 dark:bg-gray-700 rounded w-full"></div>
                        <div class="h-3 sm:h-4 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>
