<script lang="ts">
    import { page } from '$app/stores';

    let workerId = $derived($page.params.id);
</script>

<div class="max-w-4xl mx-auto">
    <div class="mb-6 sm:mb-8">
        <a href="/workers" class="text-sm text-blue-600 dark:text-blue-400 hover:underline mb-2 inline-block">← Back to Workers</a>
        <h1 class="text-2xl sm:text-3xl font-bold text-gray-900 dark:text-gray-100 mb-1 sm:mb-2">Configure Worker #{workerId}</h1>
        <p class="text-sm sm:text-base text-gray-600 dark:text-gray-400">Worker configuration — coming soon</p>
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-xl p-8 sm:p-12 text-center border border-gray-100 dark:border-gray-700">
        <span class="text-4xl mb-4 block">⚙️</span>
        <p class="text-gray-600 dark:text-gray-400 mb-2">Configuration page coming soon</p>
        <p class="text-sm text-gray-500 dark:text-gray-500">This feature will allow you to change model, provider, and other settings for this worker.</p>
    </div>
</div>
