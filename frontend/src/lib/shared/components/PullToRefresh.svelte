<script lang="ts">
    import { haptic } from '$lib/shared/utils/touch';
    
    let { onrefresh, refreshing = false, threshold = 80, children }: {
        onrefresh: () => void;
        refreshing?: boolean;
        threshold?: number;
        children: any;
    } = $props();
    
    let pullDistance = $state(0);
    let startY = $state(0);
    let isPulling = $state(false);
    let containerEl: HTMLDivElement;
    
    function onTouchStart(e: TouchEvent) {
        if (containerEl && containerEl.scrollTop > 0) return;
        startY = e.touches[0].clientY;
        isPulling = true;
    }
    
    function onTouchMove(e: TouchEvent) {
        if (!isPulling || refreshing) return;
        
        const currentY = e.touches[0].clientY;
        const delta = currentY - startY;
        
        if (delta > 0 && containerEl && containerEl.scrollTop <= 0) {
            pullDistance = Math.min(delta * 0.5, threshold * 1.5);
            e.preventDefault();
        }
    }
    
    function onTouchEnd() {
        if (!isPulling) return;
        isPulling = false;
        
        if (pullDistance >= threshold && !refreshing) {
            haptic(20);
            onrefresh();
        }
        
        pullDistance = 0;
    }
    
    let progress = $derived(Math.min(pullDistance / threshold, 1));
    let showIndicator = $derived(pullDistance > 10 || refreshing);
</script>

<div 
    bind:this={containerEl}
    class="relative overflow-hidden"
    ontouchstart={onTouchStart}
    ontouchmove={onTouchMove}
    ontouchend={onTouchEnd}
>
    {#if showIndicator}
        <div 
            class="absolute top-0 left-0 right-0 flex items-center justify-center transition-all duration-200 z-10"
            style="height: {pullDistance}px"
        >
            <div class="flex items-center gap-2 text-sm text-gray-500">
                {#if refreshing}
                    <span class="animate-spin">⏳</span>
                    <span>Refreshing...</span>
                {:else if progress >= 1}
                    <span class="rotate-180 transition-transform">↓</span>
                    <span>Release to refresh</span>
                {:else}
                    <span style="transform: rotate({progress * 180}deg)" class="transition-transform">↓</span>
                    <span>Pull to refresh</span>
                {/if}
            </div>
        </div>
    {/if}
    
    <div 
        class="transition-transform duration-200"
        style="transform: translateY({showIndicator ? pullDistance : 0}px)"
    >
        {@render children()}
    </div>
</div>
