<script lang="ts">
    import { onMount } from 'svelte';
    import { haptic, createSwipeDetector } from '$lib/shared/utils/touch';
    
    let { session }: {
        session: {
            id: string;
            title: string | null;
            source: string | null;
            message_count: number | null;
            started_at: number | null;
            ended_at: number | null;
            model: string | null;
        };
    } = $props();
    
    let swipeOffset = $state(0);
    let isSwiping = $state(false);
    let cardEl: HTMLDivElement;
    
    function formatDate(timestamp: number | null): string {
        if (!timestamp) return 'Unknown';
        const date = new Date(timestamp * 1000);
        const now = new Date();
        const diffMs = now.getTime() - date.getTime();
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMs / 3600000);
        const diffDays = Math.floor(diffMs / 86400000);

        if (diffMins < 1) return 'Just now';
        if (diffMins < 60) return `${diffMins}m ago`;
        if (diffHours < 24) return `${diffHours}h ago`;
        if (diffDays < 7) return `${diffDays}d ago`;
        return date.toLocaleDateString('id-ID');
    }

    function getSourceIcon(source: string | null): string {
        switch (source) {
            case 'telegram': return '📱';
            case 'discord': return '🎮';
            case 'cli': return '💻';
            case 'whatsapp': return '💬';
            default: return '🔗';
        }
    }

    function getStatusColor(ended_at: number | null): string {
        return ended_at ? 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400' : 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400';
    }

    function getStatus(ended_at: number | null): string {
        return ended_at ? 'Ended' : 'Active';
    }
    
    function handleTap() {
        haptic(5);
    }
    
    onMount(() => {
        if (!cardEl) return;
        
        const detector = createSwipeDetector(cardEl, {
            onSwipe: (direction, distance) => {
                if (direction === 'left') {
                    haptic(10);
                } else if (direction === 'right') {
                    haptic(5);
                }
                swipeOffset = 0;
            },
            onSwipeStart: () => { isSwiping = true; },
            onSwipeEnd: () => { isSwiping = false; swipeOffset = 0; },
            threshold: 60
        });
        
        return detector.destroy;
    });
</script>

<div class="relative overflow-hidden rounded-xl">
    <div class="absolute inset-0 flex items-center justify-end gap-2 px-4 bg-blue-500 rounded-xl">
        <span class="text-white text-sm font-medium">📋 Copy ID</span>
    </div>
    
    <div 
        bind:this={cardEl}
        class="relative bg-white dark:bg-gray-800 rounded-xl shadow-sm p-4 sm:p-5 hover:shadow-md transition-all cursor-pointer border border-gray-100 dark:border-gray-700 hover:border-blue-200 dark:hover:border-blue-800 active:bg-gray-50 dark:active:bg-gray-700 min-h-[100px] focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:outline-none"
        style="transform: translateX({swipeOffset}px); transition: {isSwiping ? 'none' : 'transform 0.2s ease'}"
        onclick={handleTap}
        role="button"
        tabindex="0"
    >
        <div class="flex justify-between items-start gap-3 sm:gap-4">
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-2">
                    <span class="text-lg">{getSourceIcon(session.source)}</span>
                    <h3 class="font-semibold text-base sm:text-lg text-gray-900 dark:text-gray-100 truncate">
                        {session.title || 'Untitled Session'}
                    </h3>
                </div>
                
                <div class="flex flex-wrap items-center gap-2 sm:gap-3 text-xs sm:text-sm text-gray-500 dark:text-gray-400">
                    <span class="flex items-center gap-1">
                        <span class="capitalize">{session.source || 'unknown'}</span>
                    </span>
                    <span class="text-gray-300 dark:text-gray-600">•</span>
                    <span class="flex items-center gap-1">
                        💬 {session.message_count || 0}
                    </span>
                    <span class="text-gray-300 dark:text-gray-600">•</span>
                    <span class="flex items-center gap-1">
                        🤖 {session.model || 'unknown'}
                    </span>
                </div>
            </div>
            
            <div class="flex flex-col items-end gap-2 shrink-0">
                <span class="text-[10px] sm:text-xs px-2 py-1 rounded-full font-medium {getStatusColor(session.ended_at)}">
                    {getStatus(session.ended_at)}
                </span>
                <span class="text-[10px] sm:text-xs text-gray-400 dark:text-gray-500">
                    {formatDate(session.started_at)}
                </span>
            </div>
        </div>
        
        <div class="mt-3 pt-3 border-t border-gray-100 dark:border-gray-700">
            <p class="text-[10px] sm:text-xs text-gray-400 dark:text-gray-500 font-mono truncate">ID: {session.id}</p>
        </div>
    </div>
</div>
