<script lang="ts">
    import { onMount } from 'svelte';
    import { apiFetch } from '$lib/shared/utils/api';

    interface MessageTarget {
        platform: string;
        id: string;
        name: string;
        type: string;
        thread_id?: string;
    }

    let targets = $state<MessageTarget[]>([]);
    let selectedTarget = $state('');
    let message = $state('');
    let sending = $state(false);
    let loading = $state(true);
    let result = $state<{ type: 'success' | 'error'; text: string } | null>(null);
    let messageHistory = $state<Array<{ text: string; target: string; time: string; success: boolean }>>([]);

    onMount(async () => {
        await loadTargets();
    });

    async function loadTargets() {
        try {
            const data = await apiFetch<{ targets: MessageTarget[] }>('/api/tools/targets');
            targets = data.targets;
            if (targets.length > 0 && !selectedTarget) {
                selectedTarget = targets[0].platform;
            }
        } catch (e) {
            console.error('Failed to load targets:', e);
            result = { type: 'error', text: 'Failed to load messaging targets' };
        } finally {
            loading = false;
        }
    }

    async function handleSend() {
        if (!message.trim()) {
            result = { type: 'error', text: 'Message cannot be empty' };
            return;
        }

        sending = true;
        result = null;

        try {
            const targetParam = selectedTarget || undefined;
            const data = await apiFetch<{
                success: boolean;
                message: string;
                platform?: string;
                chat_id?: string;
                message_id?: string;
            }>('/api/tools/send-message', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    message: message.trim(),
                    target: targetParam,
                }),
            });

            if (data.success) {
                result = { type: 'success', text: data.message || 'Message sent successfully' };
                messageHistory = [
                    {
                        text: message.trim(),
                        target: data.platform || selectedTarget,
                        time: new Date().toLocaleTimeString(),
                        success: true,
                    },
                    ...messageHistory.slice(0, 9),
                ];
                message = '';
            } else {
                result = { type: 'error', text: data.message || 'Failed to send message' };
            }
        } catch (e) {
            console.error('Failed to send message:', e);
            result = { type: 'error', text: 'Failed to send message. Check your connection.' };
        } finally {
            sending = false;
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    }
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
    <div class="flex items-center gap-3 mb-6">
        <span class="text-2xl">💬</span>
        <div>
            <h2 class="text-xl font-semibold text-gray-900">Send Message</h2>
            <p class="text-sm text-gray-500">Send a message to your agent via any connected platform</p>
        </div>
    </div>

    {#if loading}
        <div class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
    {:else}
        <div class="mb-4">
            <label for="target" class="block text-sm font-medium text-gray-700 mb-2">Target Platform</label>
            <select
                id="target"
                bind:value={selectedTarget}
                class="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-lg text-gray-900 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
            >
                {#each targets as target}
                    <option value={target.platform}>
                        {target.platform.charAt(0).toUpperCase() + target.platform.slice(1)} — {target.name} ({target.type})
                    </option>
                {/each}
                {#if targets.length === 0}
                    <option value="">No targets available</option>
                {/if}
            </select>
        </div>

        <div class="mb-4">
            <label for="message" class="block text-sm font-medium text-gray-700 mb-2">Message</label>
            <textarea
                id="message"
                bind:value={message}
                onkeydown={handleKeydown}
                placeholder="Type your message here... (Enter to send, Shift+Enter for newline)"
                rows="4"
                disabled={sending}
                class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-lg text-gray-900 placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors resize-none disabled:opacity-50"
            ></textarea>
        </div>

        <div class="flex items-center gap-3">
            <button
                onclick={handleSend}
                disabled={sending || !message.trim()}
                class="px-6 py-2.5 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            >
                {#if sending}
                    <div class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent"></div>
                    Sending...
                {:else}
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                    </svg>
                    Send
                {/if}
            </button>
            {#if message.trim()}
                <span class="text-sm text-gray-400">{message.length} chars</span>
            {/if}
        </div>

        {#if result}
            <div class="mt-4 p-3 rounded-lg {result.type === 'success' ? 'bg-green-50 text-green-800 border border-green-200' : 'bg-red-50 text-red-800 border border-red-200'}">
                {result.text}
            </div>
        {/if}

        {#if messageHistory.length > 0}
            <div class="mt-6 border-t border-gray-100 pt-4">
                <h3 class="text-sm font-medium text-gray-500 mb-3">Recent Messages</h3>
                <div class="space-y-2">
                    {#each messageHistory as entry}
                        <div class="flex items-start gap-3 text-sm">
                            <span class={entry.success ? 'text-green-500' : 'text-red-500'}>
                                {entry.success ? '✓' : '✗'}
                            </span>
                            <div class="flex-1 min-w-0">
                                <p class="text-gray-900 truncate">{entry.text}</p>
                                <p class="text-gray-400 text-xs">{entry.target} · {entry.time}</p>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    {/if}
</div>
