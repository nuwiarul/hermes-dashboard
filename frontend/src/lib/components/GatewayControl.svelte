<script lang="ts">
    import { onMount } from 'svelte';
    import { apiFetch } from '$lib/shared/utils/api';

    interface GatewayProfile {
        name: string;
        active: boolean;
        pid?: string;
    }

    interface GatewayStatus {
        running: boolean;
        pid?: string;
        uptime?: string;
        service_name: string;
        profiles: GatewayProfile[];
        raw_status: string;
    }

    let status = $state<GatewayStatus | null>(null);
    let loading = $state(true);
    let restarting = $state(false);
    let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

    onMount(async () => {
        await loadStatus();
    });

    async function loadStatus() {
        try {
            loading = true;
            status = await apiFetch<GatewayStatus>('/api/tools/gateway/status');
        } catch (e) {
            console.error('Failed to load gateway status:', e);
            message = { type: 'error', text: 'Failed to load gateway status' };
        } finally {
            loading = false;
        }
    }

    async function handleRestart() {
        if (!confirm('Restart the gateway? This will temporarily disconnect messaging platforms.')) {
            return;
        }

        restarting = true;
        message = null;

        try {
            const data = await apiFetch<{ success: boolean; message: string }>(
                '/api/tools/gateway/restart',
                { method: 'POST' }
            );

            if (data.success) {
                message = { type: 'success', text: 'Gateway restarted successfully!' };
                setTimeout(async () => {
                    await loadStatus();
                }, 3000);
            } else {
                message = { type: 'error', text: data.message || 'Failed to restart gateway' };
            }
        } catch (e) {
            console.error('Failed to restart gateway:', e);
            message = { type: 'error', text: 'Failed to restart gateway' };
        } finally {
            restarting = false;
        }
    }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
    <div class="flex items-center gap-3 mb-6">
        <span class="text-2xl">⚡</span>
        <div>
            <h2 class="text-xl font-semibold text-gray-900">Gateway Control</h2>
            <p class="text-sm text-gray-500">Monitor and restart your messaging gateway</p>
        </div>
    </div>

    {#if loading}
        <div class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
    {:else}
        <div class="bg-gray-50 rounded-lg p-4 mb-6">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="w-3 h-3 rounded-full {status?.running ? 'bg-green-500' : 'bg-red-500'}"></div>
                    <div>
                        <div class="text-sm text-gray-500">Gateway Status</div>
                        <div class="font-semibold text-gray-900">
                            {status?.running ? 'Running' : 'Stopped'}
                        </div>
                    </div>
                </div>
                {#if status?.pid}
                    <div class="text-right">
                        <div class="text-sm text-gray-500">PID</div>
                        <div class="font-mono text-sm text-gray-900">{status.pid}</div>
                    </div>
                {/if}
            </div>
            {#if status?.uptime}
                <div class="mt-3 text-sm text-gray-500">
                    Running since: <span class="font-medium text-gray-700">{status.uptime}</span>
                </div>
            {/if}
        </div>

        {#if status?.profiles && status.profiles.length > 0}
            <div class="mb-6">
                <h3 class="text-sm font-medium text-gray-500 mb-3">Profiles</h3>
                <div class="space-y-2">
                    {#each status.profiles as profile}
                        <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                            <div class="flex items-center gap-3">
                                <div class="w-2 h-2 rounded-full {profile.active ? 'bg-green-500' : 'bg-gray-300'}"></div>
                                <span class="font-medium text-gray-900">{profile.name}</span>
                                {#if profile.active}
                                    <span class="px-2 py-0.5 text-xs font-medium bg-green-100 text-green-800 rounded">active</span>
                                {/if}
                            </div>
                            {#if profile.pid}
                                <span class="text-sm font-mono text-gray-500">PID {profile.pid}</span>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        {#if message}
            <div class="mb-4 p-3 rounded-lg {message.type === 'success' ? 'bg-green-50 text-green-800 border border-green-200' : 'bg-red-50 text-red-800 border border-red-200'}">
                {message.text}
            </div>
        {/if}

        <div class="flex items-center gap-3">
            <button
                onclick={loadStatus}
                disabled={loading}
                class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50"
            >
                🔄 Refresh
            </button>
            <button
                onclick={handleRestart}
                disabled={restarting || !status?.running}
                class="px-4 py-2 text-sm font-medium text-white bg-orange-600 rounded-lg hover:bg-orange-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            >
                {#if restarting}
                    <div class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent"></div>
                    Restarting...
                {:else}
                    ⚡ Restart Gateway
                {/if}
            </button>
        </div>
    {/if}
</div>
