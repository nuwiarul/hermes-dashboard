<script lang="ts">
    import '../app.css';
    import Sidebar from '$lib/components/Sidebar.svelte';
    import Header from '$lib/components/Header.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { status, connectWebSocket, disconnectWebSocket } from '$lib/stores/status';
    
    let { children } = $props();
    
    onMount(() => {
        connectWebSocket();
    });
    
    onDestroy(() => {
        disconnectWebSocket();
    });
</script>

<div class="flex min-h-screen bg-gray-100">
    <Sidebar />
    
    <div class="flex-1 flex flex-col">
        <Header status={$status.online ? 'online' : 'offline'} model="mimo-v2.5" />
        
        <main class="flex-1 p-6">
            {@render children()}
        </main>
    </div>
</div>
