<script lang="ts">
    let { status = 'online', model = 'Unknown' }: { 
        status?: 'online' | 'offline'; 
        model?: string 
    } = $props();
</script>

<header class="bg-white border-b border-gray-200 px-6 py-4 flex justify-between items-center">
    <div>
        <span class="text-sm text-gray-500">Model:</span>
        <span class="ml-2 font-mono text-gray-900">{model}</span>
    </div>
    
    <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-green-500' : 'bg-red-500'}"></span>
        <span class="text-sm text-gray-700 capitalize">{status}</span>
    </div>
</header>
