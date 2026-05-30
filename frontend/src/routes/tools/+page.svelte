<script lang="ts">
    import ModelSwitcher from '$lib/components/ModelSwitcher.svelte';
    import SendMessage from '$lib/components/SendMessage.svelte';
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Tools</h1>
        <p class="text-gray-600">Remote control for your AI agent</p>
    </div>
    
    <div class="space-y-6">
        <!-- Model Switcher -->
        <ModelSwitcher />
        
        <!-- Send Message -->
        <SendMessage />
        
        <!-- Placeholder: Tool Manager -->
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 opacity-50">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">🛠️</span>
                <div>
                    <h2 class="text-xl font-semibold text-gray-900">Tool Manager</h2>
                    <p class="text-sm text-gray-500">Enable/disable tools (coming soon)</p>
                </div>
            </div>
            <div class="text-gray-400 text-center py-4">
                Coming in Task 10.2
            </div>
        </div>
        
        <!-- Placeholder: Gateway Control -->
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 opacity-50">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">⚡</span>
                <div>
                    <h2 class="text-xl font-semibold text-gray-900">Gateway Control</h2>
                    <p class="text-sm text-gray-500">Restart/status gateway (coming soon)</p>
                </div>
            </div>
            <div class="text-gray-400 text-center py-4">
                Coming in Task 10.4
            </div>
        </div>
    </div>
</div>
