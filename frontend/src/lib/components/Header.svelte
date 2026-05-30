<script lang="ts">
    import { auth } from '$lib/stores/auth';

    let { status = 'online', model = 'Unknown' }: { 
        status?: 'online' | 'offline'; 
        model?: string 
    } = $props();

    async function handleLogout() {
        await auth.logout();
    }
</script>

<header class="bg-white border-b border-gray-200 px-6 py-4 flex justify-between items-center">
    <div>
        <span class="text-sm text-gray-500">Model:</span>
        <span class="ml-2 font-mono text-gray-900">{model}</span>
    </div>
    
    <div class="flex items-center gap-4">
        <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-green-500' : 'bg-red-500'}"></span>
            <span class="text-sm text-gray-700 capitalize">{status}</span>
        </div>
        
        {#if $auth.username}
            <span class="text-sm text-gray-500">{$auth.username}</span>
        {/if}
        
        <button
            onclick={handleLogout}
            class="text-sm text-gray-500 hover:text-red-600 transition cursor-pointer px-3 py-1 rounded hover:bg-red-50"
        >
            Logout
        </button>
    </div>
</header>
