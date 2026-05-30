<script lang="ts">
    import ModelSwitcher from '$lib/components/ModelSwitcher.svelte';
    import ToolManager from '$lib/components/ToolManager.svelte';
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Tools</h1>
        <p class="text-gray-600">Remote control for your AI agent</p>
    </div>
    
    <div class="space-y-6">
        <!-- Model Switcher -->
        <ModelSwitcher />
        
        <!-- Tool Manager -->
        <ToolManager />
        
        <!-- Placeholder for future tools -->
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 opacity-50">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">💬</span>
                <div>
                    <h2 class="text-xl font-semibold text-gray-900">Send Message</h2>
                    <p class="text-sm text-gray-500">Send messages to your agent (coming soon)</p>
                </div>
            </div>
            <div class="text-gray-400 text-center py-4">
                Coming in Task 10.3
            </div>
        </div>
        
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
