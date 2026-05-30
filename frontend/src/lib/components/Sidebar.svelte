<script lang="ts">
    import { page } from '$app/stores';
    
    const navItems = [
        { href: '/', label: 'Dashboard', icon: '📊' },
        { href: '/sessions', label: 'Sessions', icon: '💬' },
        { href: '/cron', label: 'Cron Jobs', icon: '⏰' },
        { href: '/tools', label: 'Tools', icon: '🔧' },
        { href: '/settings', label: 'Settings', icon: '⚙️' },
    ];
    
    let sidebarOpen = $state(false);
</script>

<!-- Mobile toggle button -->
<button 
    onclick={() => sidebarOpen = !sidebarOpen}
    class="lg:hidden fixed top-4 left-4 z-50 p-2 bg-gray-900 text-white rounded-lg"
>
    {sidebarOpen ? '✕' : '☰'}
</button>

<!-- Overlay for mobile -->
{#if sidebarOpen}
    <button 
        onclick={() => sidebarOpen = false}
        class="lg:hidden fixed inset-0 bg-black/50 z-40"
        aria-label="Close sidebar"
    ></button>
{/if}

<!-- Sidebar -->
<aside class="fixed lg:static inset-y-0 left-0 z-40 w-64 bg-gray-900 text-white transform transition-transform duration-200 ease-in-out {sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}">
    <div class="p-4">
        <h1 class="text-2xl font-bold">🤖 Hermes</h1>
        <p class="text-gray-400 text-sm">Dashboard</p>
    </div>
    
    <nav class="px-4">
        {#each navItems as item}
            <a 
                href={item.href}
                onclick={() => sidebarOpen = false}
                class="flex items-center gap-3 px-4 py-3 rounded-lg mb-2 transition-colors
                       {$page.url.pathname === item.href 
                         ? 'bg-blue-600 text-white' 
                         : 'text-gray-300 hover:bg-gray-800'}"
            >
                <span>{item.icon}</span>
                <span>{item.label}</span>
            </a>
        {/each}
    </nav>
</aside>
