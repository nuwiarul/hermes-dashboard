<script lang="ts">
    import { apiFetch } from '$lib/shared/utils/api';
    
    interface ModelInfo {
        default: string;
        provider: string;
        fallback?: string;
        base_url?: string;
    }
    
    interface AvailableModel {
        name: string;
        provider: string;
        description?: string;
    }
    
    let currentModel = $state<ModelInfo | null>(null);
    let availableModels = $state<AvailableModel[]>([]);
    let selectedModel = $state('');
    let loading = $state(true);
    let switching = $state(false);
    let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);
    let showConfirm = $state(false);
    
    onMount(async () => {
        await loadModels();
    });
    
    async function loadModels() {
        try {
            const data = await apiFetch<{ current: ModelInfo; available: AvailableModel[] }>('/api/tools/models');
            currentModel = data.current;
            availableModels = data.available;
            selectedModel = data.current.default;
        } catch (e) {
            console.error('Failed to load models:', e);
            message = { type: 'error', text: 'Failed to load models' };
        } finally {
            loading = false;
        }
    }
    
    function handleSelect(model: string) {
        if (model === currentModel?.default) {
            message = { type: 'error', text: 'Already using this model' };
            return;
        }
        selectedModel = model;
        showConfirm = true;
    }
    
    async function confirmSwitch() {
        showConfirm = false;
        switching = true;
        message = null;
        
        try {
            const data = await apiFetch<{ success: boolean; message: string }>('/api/tools/switch-model', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ model: selectedModel }),
            });
            
            if (data.success) {
                message = { type: 'success', text: data.message };
                await loadModels();
            } else {
                message = { type: 'error', text: data.message };
            }
        } catch (e) {
            console.error('Failed to switch model:', e);
            message = { type: 'error', text: 'Failed to switch model' };
        } finally {
            switching = false;
        }
    }
    
    function cancelSwitch() {
        showConfirm = false;
        selectedModel = currentModel?.default || '';
    }
    
    import { onMount } from 'svelte';
</script>

<div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
    <div class="flex items-center gap-3 mb-6">
        <span class="text-2xl">🔄</span>
        <div>
            <h2 class="text-xl font-semibold text-gray-900">Switch Model</h2>
            <p class="text-sm text-gray-500">Change the AI model used by your agent</p>
        </div>
    </div>
    
    {#if loading}
        <div class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        </div>
    {:else}
        <!-- Current Model -->
        <div class="bg-gray-50 rounded-lg p-4 mb-6">
            <div class="text-sm text-gray-500 mb-1">Current Model</div>
            <div class="flex items-center gap-2">
                <span class="text-lg font-semibold text-gray-900">{currentModel?.default}</span>
                <span class="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded-full">
                    {currentModel?.provider}
                </span>
            </div>
            {#if currentModel?.fallback}
                <div class="text-sm text-gray-500 mt-2">
                    Fallback: <span class="font-medium">{currentModel.fallback}</span>
                </div>
            {/if}
        </div>
        
        <!-- Message -->
        {#if message}
            <div class="mb-4 p-3 rounded-lg {message.type === 'success' ? 'bg-green-50 text-green-800' : 'bg-red-50 text-red-800'}">
                {message.text}
            </div>
        {/if}
        
        <!-- Model List -->
        <div class="space-y-3">
            {#each availableModels as model}
                <button
                    onclick={() => handleSelect(model.name)}
                    disabled={switching || model.name === currentModel?.default}
                    class="w-full text-left p-4 rounded-lg border transition-all {model.name === currentModel?.default 
                        ? 'border-blue-500 bg-blue-50 cursor-default' 
                        : 'border-gray-200 hover:border-blue-300 hover:bg-gray-50 cursor-pointer'}"
                >
                    <div class="flex items-center justify-between">
                        <div>
                            <div class="flex items-center gap-2">
                                <span class="font-medium text-gray-900">{model.name}</span>
                                <span class="px-2 py-0.5 text-xs font-medium bg-gray-100 text-gray-600 rounded">
                                    {model.provider}
                                </span>
                                {#if model.name === currentModel?.default}
                                    <span class="px-2 py-0.5 text-xs font-medium bg-blue-100 text-blue-800 rounded">
                                        Active
                                    </span>
                                {/if}
                            </div>
                            {#if model.description}
                                <p class="text-sm text-gray-500 mt-1">{model.description}</p>
                            {/if}
                        </div>
                        {#if model.name !== currentModel?.default}
                            <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                            </svg>
                        {/if}
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<!-- Confirmation Modal -->
{#if showConfirm}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div class="bg-white rounded-xl shadow-xl p-6 max-w-md w-full mx-4">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Switch Model?</h3>
            <p class="text-gray-600 mb-6">
                Change model from <strong>{currentModel?.default}</strong> to <strong>{selectedModel}</strong>?
            </p>
            <div class="flex gap-3 justify-end">
                <button
                    onclick={cancelSwitch}
                    class="px-4 py-2 text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors"
                >
                    Cancel
                </button>
                <button
                    onclick={confirmSwitch}
                    disabled={switching}
                    class="px-4 py-2 text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
                >
                    {switching ? 'Switching...' : 'Switch'}
                </button>
            </div>
        </div>
    </div>
{/if}
