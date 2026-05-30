<script lang="ts">
    let { title, value, icon, trend = 'neutral' }: {
        title: string;
        value: string | number;
        icon: string;
        trend?: 'up' | 'down' | 'neutral';
    } = $props();
</script>

<div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition-shadow">
    <div class="flex items-center justify-between mb-4">
        <span class="text-3xl">{icon}</span>
        {#if trend === 'up'}
            <span class="text-green-500 text-sm font-medium">↑ +12%</span>
        {:else if trend === 'down'}
            <span class="text-red-500 text-sm font-medium">↓ -5%</span>
        {/if}
    </div>
    <p class="text-gray-500 text-sm">{title}</p>
    <p class="text-3xl font-bold mt-1 text-gray-900">{value}</p>
</div>
