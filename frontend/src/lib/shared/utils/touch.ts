/**
 * Touch-friendly utilities for mobile interactions
 */

/** Trigger haptic feedback if supported */
export function haptic(pattern: number | number[] = 10): void {
    if ('vibrate' in navigator) {
        navigator.vibrate(pattern);
    }
}

/** Swipe direction */
export type SwipeDirection = 'left' | 'right' | 'up' | 'down';

/** Swipe callback */
export type SwipeCallback = (direction: SwipeDirection, distance: number) => void;

/** Create swipe gesture detector for an element */
export function createSwipeDetector(
    element: HTMLElement,
    callbacks: {
        onSwipe?: SwipeCallback;
        onSwipeStart?: () => void;
        onSwipeEnd?: () => void;
        threshold?: number;
    } = {}
): { destroy: () => void } {
    const threshold = callbacks.threshold ?? 50;
    let startX = 0;
    let startY = 0;
    let isDragging = false;

    function onTouchStart(e: TouchEvent) {
        startX = e.touches[0].clientX;
        startY = e.touches[0].clientY;
        isDragging = true;
        callbacks.onSwipeStart?.();
    }

    function onTouchMove(e: TouchEvent) {
        if (!isDragging) return;
        // Prevent vertical scroll when swiping horizontally
        const deltaX = Math.abs(e.touches[0].clientX - startX);
        const deltaY = Math.abs(e.touches[0].clientY - startY);
        if (deltaX > deltaY && deltaX > 10) {
            e.preventDefault();
        }
    }

    function onTouchEnd(e: TouchEvent) {
        if (!isDragging) return;
        isDragging = false;
        callbacks.onSwipeEnd?.();

        const endX = e.changedTouches[0].clientX;
        const endY = e.changedTouches[0].clientY;
        const deltaX = endX - startX;
        const deltaY = endY - startY;

        const absDeltaX = Math.abs(deltaX);
        const absDeltaY = Math.abs(deltaY);

        if (absDeltaX < threshold && absDeltaY < threshold) return;

        let direction: SwipeDirection;
        if (absDeltaX > absDeltaY) {
            direction = deltaX > 0 ? 'right' : 'left';
        } else {
            direction = deltaY > 0 ? 'down' : 'up';
        }

        callbacks.onSwipe?.(direction, absDeltaX > absDeltaY ? absDeltaX : absDeltaY);
    }

    element.addEventListener('touchstart', onTouchStart, { passive: true });
    element.addEventListener('touchmove', onTouchMove, { passive: false });
    element.addEventListener('touchend', onTouchEnd, { passive: true });

    return {
        destroy() {
            element.removeEventListener('touchstart', onTouchStart);
            element.removeEventListener('touchmove', onTouchMove);
            element.removeEventListener('touchend', onTouchEnd);
        }
    };
}
