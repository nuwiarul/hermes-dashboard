<script lang="ts">
    import '../app.css';
    import Sidebar from '$lib/components/Sidebar.svelte';
    import Header from '$lib/components/Header.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { status, connectWebSocket, disconnectWebSocket } from '$lib/stores/status';
    import { auth } from '$lib/stores/auth';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';

    let { children } = $props();

    let isLoginPage = $derived($page.url.pathname === '/login');

    // Check auth status on mount (validates cookie via /api/auth/me)
    onMount(() => {
        auth.checkAuth();
    });

    // Redirect to login if not authenticated (after loading)
    $effect(() => {
        if (!$auth.isLoading && !$auth.isAuthenticated && !isLoginPage) {
            goto('/login');
        }
    });

    // Connect WebSocket when authenticated
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
    <!-- Login page: no sidebar/header -->
    {@render children()}
{:else if $auth.isLoading}
    <!-- Checking auth status -->
    <div class="min-h-screen bg-gray-50 flex items-center justify-center">
        <div class="text-gray-400">Loading...</div>
    </div>
{:else if $auth.isAuthenticated}
    <!-- Authenticated: show full layout -->
    <div class="flex min-h-screen bg-gray-100">
        <Sidebar />
        <div class="flex-1 flex flex-col min-w-0">
            <Header status={$status.online ? 'online' : 'offline'} model="mimo-v2.5" />
            <main class="flex-1 p-3 sm:p-4 md:p-6 overflow-auto">
                {@render children()}
            </main>
        </div>
    </div>
{:else}
    <!-- Not authenticated: redirecting -->
    <div class="min-h-screen bg-gray-50 flex items-center justify-center">
        <div class="text-gray-400">Redirecting to login...</div>
    </div>
{/if}
