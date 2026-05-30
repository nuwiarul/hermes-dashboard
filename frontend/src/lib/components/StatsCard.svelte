<script lang="ts">
    import { haptic } from '$lib/shared/utils/touch';
    
    let { title, value, icon, trend = 'neutral' }: {
        title: string;
        value: string | number;
        icon: string;
        trend?: 'up' | 'down' | 'neutral';
    } = $props();
    
    function handleTap() {
        haptic(5);
    }
</script>

<button 
    onclick={handleTap}
    class="w-full text-left bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-5 md:p-6 hover:shadow-md transition-all active:scale-[0.98] active:bg-gray-50 dark:active:bg-gray-700 min-h-[80px] sm:min-h-[100px] cursor-pointer border border-gray-100 dark:border-gray-700 focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:outline-none"
>
    <div class="flex items-center justify-between mb-2 sm:mb-3 md:mb-4">
        <span class="text-xl sm:text-2xl md:text-3xl">{icon}</span>
        {#if trend === 'up'}
            <span class="text-green-500 text-xs sm:text-sm font-medium">↑ +12%</span>
        {:else if trend === 'down'}
            <span class="text-red-500 text-xs sm:text-sm font-medium">↓ -5%</span>
        {/if}
    </div>
    <p class="text-gray-500 dark:text-gray-400 text-xs sm:text-sm">{title}</p>
    <p class="text-xl sm:text-2xl md:text-3xl font-bold mt-0.5 sm:mt-1 text-gray-900 dark:text-gray-100">{value}</p>
</button>
