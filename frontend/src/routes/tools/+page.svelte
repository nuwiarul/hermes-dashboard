<script lang="ts">
    import ModelSwitcher from '$lib/components/ModelSwitcher.svelte';
    import ToolManager from '$lib/components/ToolManager.svelte';
    import SendMessage from '$lib/components/SendMessage.svelte';
    import GatewayControl from '$lib/components/GatewayControl.svelte';
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 mb-1 sm:mb-2">Tools</h1>
        <p class="text-sm sm:text-base text-gray-600">Remote control for your AI agent</p>
    </div>
    
    <div class="space-y-4 sm:space-y-6">
        <ModelSwitcher />
        <ToolManager />
        <SendMessage />
        <GatewayControl />
    </div>
</div>
