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
    class="lg:hidden fixed top-3 left-3 z-50 p-2.5 bg-gray-900 text-white rounded-lg shadow-lg active:scale-95 transition-transform"
    aria-label={sidebarOpen ? 'Close menu' : 'Open menu'}
>
    {sidebarOpen ? '✕' : '☰'}
</button>

<!-- Overlay for mobile -->
{#if sidebarOpen}
    <button 
        onclick={() => sidebarOpen = false}
        class="lg:hidden fixed inset-0 bg-black/50 z-40 transition-opacity"
        aria-label="Close sidebar"
    ></button>
{/if}

<!-- Sidebar -->
<aside class="fixed lg:static inset-y-0 left-0 z-40 w-64 bg-gray-900 text-white transform transition-transform duration-200 ease-in-out {sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}">
    <div class="p-4">
        <h1 class="text-xl sm:text-2xl font-bold">🤖 Hermes</h1>
        <p class="text-gray-400 text-xs sm:text-sm">Dashboard</p>
    </div>
    
    <nav class="px-3">
        {#each navItems as item}
            <a 
                href={item.href}
                onclick={() => sidebarOpen = false}
                class="flex items-center gap-3 px-4 py-3 rounded-lg mb-1 transition-colors min-h-[44px]
                       {$page.url.pathname === item.href 
                         ? 'bg-blue-600 text-white' 
                         : 'text-gray-300 hover:bg-gray-800 active:bg-gray-700'}"
            >
                <span class="text-lg">{item.icon}</span>
                <span class="text-sm sm:text-base">{item.label}</span>
            </a>
        {/each}
    </nav>
</aside>
