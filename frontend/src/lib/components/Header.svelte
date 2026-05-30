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

<header class="bg-white border-b border-gray-200 px-3 sm:px-4 md:px-6 py-3 sm:py-4 flex justify-between items-center gap-2">
    <div class="flex items-center gap-2 min-w-0">
        <span class="text-xs sm:text-sm text-gray-500 hidden sm:inline">Model:</span>
        <span class="font-mono text-xs sm:text-sm text-gray-900 truncate">{model}</span>
    </div>
    
    <div class="flex items-center gap-2 sm:gap-4 shrink-0">
        <div class="flex items-center gap-1.5 sm:gap-2">
            <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-green-500' : 'bg-red-500'}"></span>
            <span class="text-xs sm:text-sm text-gray-700 capitalize hidden sm:inline">{status}</span>
        </div>
        
        {#if $auth.username}
            <span class="text-xs sm:text-sm text-gray-500 hidden md:inline">{$auth.username}</span>
        {/if}
        
        <button
            onclick={handleLogout}
            class="text-xs sm:text-sm text-gray-500 hover:text-red-600 transition cursor-pointer px-2 sm:px-3 py-1 rounded hover:bg-red-50"
        >
            Logout
        </button>
    </div>
</header>
