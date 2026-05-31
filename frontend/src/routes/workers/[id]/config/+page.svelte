<script lang="ts">
    import { page } from '$app/stores';
    import { apiFetch } from '$lib/shared/utils/api';
    import { onMount } from 'svelte';

    let workerId = $derived($page.params.id);

    interface Worker {
        id: number;
        name: string;
        ip: string;
        config: string;
    }

    interface Model {
        name: string;
        provider: string;
        description: string;
    }

    let worker = $state<Worker | null>(null);
    let models = $state<Model[]>([]);
    let loading = $state(true);
    let saving = $state(false);
    let message = $state<{ type: 'success' | 'error'; text: string } | null>(null);

    // Form state
    let selectedModel = $state('');
    let selectedProvider = $state('');
    let maxTokens = $state(4096);
    let temperature = $state(0.7);

    async function fetchWorker() {
        try {
            const data = await apiFetch<Worker>(`/api/workers/${workerId}`);
            worker = data;
            
            // Parse existing config
            if (data.config && data.config !== '{}') {
                const config = JSON.parse(data.config);
                selectedModel = config.model || '';
                selectedProvider = config.provider || '';
                maxTokens = config.max_tokens || 4096;
                temperature = config.temperature || 0.7;
            }
        } catch (e: any) {
            message = { type: 'error', text: e.message || 'Failed to load worker' };
        }
    }

    async function fetchModels() {
        try {
            const data = await apiFetch<{ available: Model[] }>('/api/tools/models');
            models = data.available;
        } catch (e: any) {
            // Models endpoint might not be available, use defaults
            models = [
                { name: 'mimo-v2.5', provider: 'xiaomi', description: 'Xiaomi MiMo v2.5' },
                { name: 'deepseek-v4-flash', provider: 'deepseek', description: 'DeepSeek V4 Flash' },
                { name: 'claude-sonnet-4', provider: 'anthropic', description: 'Claude Sonnet 4' },
            ];
        }
    }

    function onModelChange() {
        const model = models.find(m => m.name === selectedModel);
        if (model) {
            selectedProvider = model.provider;
        }
    }

    async function applyConfig() {
        if (!worker) return;

        saving = true;
        message = null;

        try {
            await apiFetch(`/api/workers/${worker.id}/config`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    model: selectedModel || undefined,
                    provider: selectedProvider || undefined,
                    max_tokens: maxTokens,
                    temperature: temperature,
                }),
            });

            message = { type: 'success', text: 'Configuration applied successfully!' };
            
            // Refresh worker data
            await fetchWorker();
        } catch (e: any) {
            message = { type: 'error', text: e.message || 'Failed to apply configuration' };
        } finally {
            saving = false;
        }
    }

    onMount(async () => {
        loading = true;
        await Promise.all([fetchWorker(), fetchModels()]);
        loading = false;
    });
</script>

<div class="max-w-2xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <a href="/workers" class="text-sm text-blue-600 dark:text-blue-400 hover:underline mb-2 inline-block">← Back to Workers</a>
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">
            Configure {worker?.name || `Worker #${workerId}`}
        </h1>
        <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Change model, provider, and other settings</p>
    </div>

    {#if loading}
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-100 dark:border-gray-700 animate-pulse">
            <div class="space-y-4">
                <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
                <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
                <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
            </div>
        </div>
    {:else if message}
        <div class="mb-4 p-4 rounded-xl {message.type === 'success' ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800' : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'}">
            <p class="{message.type === 'success' ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}">
                {message.type === 'success' ? '✓' : '⚠️'} {message.text}
            </p>
        </div>
    {/if}

    {#if worker}
        <form onsubmit={(e) => { e.preventDefault(); applyConfig(); }} class="space-y-6">
            <!-- Model Selection -->
            <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-100 dark:border-gray-700">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">Model Settings</h2>
                
                <div class="space-y-4">
                    <!-- Model Dropdown -->
                    <div>
                        <label for="model" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Model</label>
                        <select
                            id="model"
                            bind:value={selectedModel}
                            onchange={onModelChange}
                            class="w-full px-4 py-2.5 bg-gray-50 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        >
                            <option value="">Select model...</option>
                            {#each models as model}
                                <option value={model.name}>{model.name} — {model.description}</option>
                            {/each}
                        </select>
                    </div>

                    <!-- Provider Dropdown -->
                    <div>
                        <label for="provider" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Provider</label>
                        <select
                            id="provider"
                            bind:value={selectedProvider}
                            class="w-full px-4 py-2.5 bg-gray-50 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        >
                            <option value="">Select provider...</option>
                            <option value="xiaomi">Xiaomi</option>
                            <option value="deepseek">DeepSeek</option>
                            <option value="anthropic">Anthropic</option>
                            <option value="openai">OpenAI</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- Advanced Settings -->
            <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-100 dark:border-gray-700">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">Advanced Settings</h2>
                
                <div class="space-y-4">
                    <!-- Max Tokens -->
                    <div>
                        <label for="maxTokens" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            Max Tokens: {maxTokens.toLocaleString()}
                        </label>
                        <input
                            type="range"
                            id="maxTokens"
                            min="256"
                            max="32768"
                            step="256"
                            bind:value={maxTokens}
                            class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
                        />
                        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-1">
                            <span>256</span>
                            <span>32,768</span>
                        </div>
                    </div>

                    <!-- Temperature -->
                    <div>
                        <label for="temperature" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            Temperature: {temperature.toFixed(2)}
                        </label>
                        <input
                            type="range"
                            id="temperature"
                            min="0"
                            max="2"
                            step="0.1"
                            bind:value={temperature}
                            class="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
                        />
                        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-1">
                            <span>0 (Precise)</span>
                            <span>2 (Creative)</span>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Apply Button -->
            <div class="flex gap-3">
                <button
                    type="submit"
                    disabled={saving}
                    class="flex-1 sm:flex-none px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-medium rounded-lg transition-colors"
                >
                    {#if saving}
                        <span class="flex items-center justify-center gap-2">
                            <svg class="animate-spin h-4 w-4" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            Applying...
                        </span>
                    {:else}
                        Apply Configuration
                    {/if}
                </button>
                <a
                    href="/workers"
                    class="px-6 py-3 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 font-medium rounded-lg transition-colors"
                >
                    Cancel
                </a>
            </div>
        </form>
    {/if}
</div>
