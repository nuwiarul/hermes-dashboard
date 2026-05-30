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
    let tabletExpanded = $state(false);
    
    function isActive(href: string): boolean {
        if (href === '/') return $page.url.pathname === '/';
        return $page.url.pathname.startsWith(href);
    }
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
<aside 
    class="fixed lg:static inset-y-0 left-0 z-40 bg-gray-900 text-white transform transition-all duration-200 ease-in-out
           {sidebarOpen ? 'translate-x-0 w-64' : '-translate-x-full lg:translate-x-0'}
           {tabletExpanded ? 'md:w-64' : 'md:w-16'}
           lg:w-64"
    onmouseenter={() => tabletExpanded = true}
    onmouseleave={() => tabletExpanded = false}
    role="navigation"
    aria-label="Main navigation"
>
    <!-- Logo -->
    <div class="p-4 flex items-center gap-3">
        <span class="text-2xl shrink-0">🤖</span>
        <div class="overflow-hidden transition-all duration-200 {tabletExpanded ? 'md:w-auto' : 'md:w-0'} lg:w-auto">
            <h1 class="text-xl sm:text-2xl font-bold whitespace-nowrap">Hermes</h1>
            <p class="text-gray-400 text-xs sm:text-sm whitespace-nowrap">Dashboard</p>
        </div>
    </div>
    
    <!-- Navigation -->
    <nav class="px-3 mt-2">
        {#each navItems as item}
            <a 
                href={item.href}
                onclick={() => sidebarOpen = false}
                class="group relative flex items-center gap-3 px-3 py-3 rounded-lg mb-1 transition-all duration-150 min-h-[44px]
                       {isActive(item.href) 
                         ? 'bg-blue-600/20 text-blue-400' 
                         : 'text-gray-300 hover:bg-gray-800 active:bg-gray-700'}"
                title={item.label}
            >
                <!-- Active indicator bar -->
                {#if isActive(item.href)}
                    <div class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-6 bg-blue-500 rounded-r-full"></div>
                {/if}
                
                <span class="text-lg shrink-0 ml-1">{item.icon}</span>
                <span class="text-sm sm:text-base whitespace-nowrap overflow-hidden transition-all duration-200
                             {tabletExpanded ? 'md:w-auto md:opacity-100 md:block' : 'md:w-0 md:opacity-0 md:hidden'} lg:w-auto lg:opacity-100 lg:block">
                    {item.label}
                </span>
            </a>
        {/each}
    </nav>
    
    <!-- Version footer -->
    <div class="absolute bottom-4 left-0 right-0 px-4 text-center">
        <span class="text-xs text-gray-500 {tabletExpanded ? 'md:inline' : 'md:hidden'} lg:inline">v0.1.0</span>
    </div>
</aside>
