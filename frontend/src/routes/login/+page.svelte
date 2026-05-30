<script lang="ts">
  import { auth } from '$lib/stores/auth';
  import { theme } from '$lib/stores/theme';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let username = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  onMount(() => { theme.init(); });

  async function handleLogin(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    const result = await auth.login(username, password);
    if (result.success) {
      await auth.checkAuth();
      goto('/');
    } else {
      error = result.message;
    }
    loading = false;
  }

  let themeIcon = $derived($theme === 'dark' ? '🌙' : $theme === 'light' ? '☀️' : '💻');
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center px-4 transition-colors">
  <!-- Theme toggle (top-right) -->
  <button
    onclick={() => theme.toggle()}
    class="fixed top-4 right-4 p-2.5 rounded-lg bg-white dark:bg-gray-800 shadow-md hover:shadow-lg transition-all min-h-[44px] min-w-[44px] flex items-center justify-center border border-gray-200 dark:border-gray-700"
    aria-label="Toggle theme"
  >
    <span class="text-lg">{themeIcon}</span>
  </button>

  <div class="w-full max-w-md">
    <div class="text-center mb-8">
      <div class="text-5xl mb-4">🤖</div>
      <h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">Hermes Dashboard</h1>
      <p class="text-gray-500 dark:text-gray-400 mt-2">Sign in to your account</p>
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-lg p-8 border border-gray-100 dark:border-gray-700">
      <form onsubmit={handleLogin}>
        {#if error}
          <div class="mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-700 dark:text-red-400 text-sm">{error}</div>
        {/if}

        <div class="mb-4">
          <label for="username" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Username</label>
          <input
            id="username" type="text" bind:value={username} required
            class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500"
            placeholder="Enter username"
          />
        </div>

        <div class="mb-6">
          <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Password</label>
          <input
            id="password" type="password" bind:value={password} required
            class="w-full px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500"
            placeholder="Enter password"
          />
        </div>

        <button
          type="submit" disabled={loading}
          class="w-full py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-semibold rounded-lg transition cursor-pointer min-h-[44px]"
        >
          {#if loading}
            <span class="inline-flex items-center gap-2">
              <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
              </svg>
              Signing in...
            </span>
          {:else}
            Sign In
          {/if}
        </button>
      </form>
    </div>

    <p class="text-center text-gray-400 dark:text-gray-500 text-sm mt-6">Hermes Agent Dashboard v0.1.0</p>
  </div>
</div>
