import { describe, it, expect, vi } from 'vitest';
import { haptic, createSwipeDetector } from './touch';

describe('touch utility - haptic', () => {
    it('calls navigator.vibrate when available', () => {
        const mockVibrate = vi.fn();
        Object.defineProperty(navigator, 'vibrate', { value: mockVibrate, writable: true });
        haptic(10);
        expect(mockVibrate).toHaveBeenCalledWith(10);
    });

    it('uses default value of 10', () => {
        const mockVibrate = vi.fn();
        Object.defineProperty(navigator, 'vibrate', { value: mockVibrate, writable: true });
        haptic();
        expect(mockVibrate).toHaveBeenCalledWith(10);
    });

    it('accepts array pattern', () => {
        const mockVibrate = vi.fn();
        Object.defineProperty(navigator, 'vibrate', { value: mockVibrate, writable: true });
        haptic([10, 50, 10]);
        expect(mockVibrate).toHaveBeenCalledWith([10, 50, 10]);
    });
});

describe('touch utility - createSwipeDetector', () => {
    it('returns destroy function', () => {
        const el = document.createElement('div');
        const detector = createSwipeDetector(el, {});
        expect(typeof detector.destroy).toBe('function');
        detector.destroy();
    });

    it('calls onSwipeStart on touchstart', () => {
        const el = document.createElement('div');
        const onSwipeStart = vi.fn();
        createSwipeDetector(el, { onSwipeStart });

        const event = new Event('touchstart') as any;
        event.touches = [{ clientX: 100, clientY: 100 }];
        el.dispatchEvent(event);

        expect(onSwipeStart).toHaveBeenCalled();
    });

    it('destroy removes event listeners', () => {
        const el = document.createElement('div');
        const onSwipeStart = vi.fn();
        const detector = createSwipeDetector(el, { onSwipeStart });

        detector.destroy();

        const event = new Event('touchstart') as any;
        event.touches = [{ clientX: 100, clientY: 100 }];
        el.dispatchEvent(event);

        expect(onSwipeStart).not.toHaveBeenCalled();
    });

    it('does not throw when destroying', () => {
        const el = document.createElement('div');
        const detector = createSwipeDetector(el, {});
        expect(() => detector.destroy()).not.toThrow();
    });
});
