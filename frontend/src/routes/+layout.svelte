<script lang="ts">
    import '../app.css';
    import Sidebar from '$lib/components/Sidebar.svelte';
    import Header from '$lib/components/Header.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { status, connectWebSocket, disconnectWebSocket } from '$lib/stores/status';
    import { auth } from '$lib/stores/auth';
    import { theme } from '$lib/stores/theme';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';

    let { children } = $props();

    let isLoginPage = $derived($page.url.pathname === '/login');

    onMount(() => {
        theme.init();
        auth.checkAuth();
    });

    $effect(() => {
        if (!$auth.isLoading && !$auth.isAuthenticated && !isLoginPage) {
            goto('/login');
        }
    });

    $effect(() => {
        if ($auth.isAuthenticated && !isLoginPage) {
            connectWebSocket();
        }
    });

    onDestroy(() => {
        disconnectWebSocket();
    });
</script>

{#if isLoginPage}
    {@render children()}
{:else if $auth.isLoading}
    <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center">
        <div class="text-gray-400 dark:text-gray-500">Loading...</div>
    </div>
{:else if $auth.isAuthenticated}
    <div class="flex min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors">
        <Sidebar />
        <div class="hidden md:block lg:hidden w-16 shrink-0"></div>
        <div class="flex-1 flex flex-col min-w-0">
            <Header status={$status.online ? 'online' : 'offline'} model="mimo-v2.5" />
            <main class="flex-1 p-3 sm:p-4 md:p-6 overflow-auto">
                {@render children()}
            </main>
        </div>
    </div>
{:else}
    <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center">
        <div class="text-gray-400 dark:text-gray-500">Redirecting to login...</div>
    </div>
{/if}
