<script lang="ts">
    import '../app.css';
    import Sidebar from '$lib/components/Sidebar.svelte';
    import Header from '$lib/components/Header.svelte';
    
    let { children } = $props();
</script>

<div class="flex min-h-screen bg-gray-100">
    <Sidebar />
    
    <div class="flex-1 flex flex-col">
        <Header status="online" model="mimo-v2.5" />
        
        <main class="flex-1 p-6">
            {@render children()}
        </main>
    </div>
</div>
