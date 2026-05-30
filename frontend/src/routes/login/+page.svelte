<script lang="ts">
  import { auth } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let username = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleLogin(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';

    const result = await auth.login(username, password);

    if (result.success) {
      // Cookie is set, now check auth status
      await auth.checkAuth();
      goto('/');
    } else {
      error = result.message;
    }

    loading = false;
  }
</script>

<div class="min-h-screen bg-gray-50 flex items-center justify-center px-4">
  <div class="w-full max-w-md">
    <!-- Logo & Title -->
    <div class="text-center mb-8">
      <div class="text-5xl mb-4">🤖</div>
      <h1 class="text-3xl font-bold text-gray-900">Hermes Dashboard</h1>
      <p class="text-gray-500 mt-2">Sign in to your account</p>
    </div>

    <!-- Login Card -->
    <div class="bg-white rounded-2xl shadow-lg p-8">
      <form onsubmit={handleLogin}>
        <!-- Error Message -->
        {#if error}
          <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
            {error}
          </div>
        {/if}

        <!-- Username -->
        <div class="mb-4">
          <label for="username" class="block text-sm font-medium text-gray-700 mb-1">
            Username
          </label>
          <input
            id="username"
            type="text"
            bind:value={username}
            required
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="Enter username"
          />
        </div>

        <!-- Password -->
        <div class="mb-6">
          <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
            Password
          </label>
          <input
            id="password"
            type="password"
            bind:value={password}
            required
            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
            placeholder="Enter password"
          />
        </div>

        <!-- Submit Button -->
        <button
          type="submit"
          disabled={loading}
          class="w-full py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-semibold rounded-lg transition cursor-pointer"
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

    <!-- Footer -->
    <p class="text-center text-gray-400 text-sm mt-6">
      Hermes Agent Dashboard v0.1.0
    </p>
  </div>
</div>
