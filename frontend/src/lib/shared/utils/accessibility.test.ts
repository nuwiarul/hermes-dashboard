import { describe, it, expect } from 'vitest';
import { readFileSync, existsSync } from 'fs';
import { resolve } from 'path';

/**
 * Accessibility Tests for Hermes Dashboard
 * 
 * Tests verify HTML structure, semantic elements, ARIA attributes,
 * responsive design, dark mode, touch targets, keyboard navigation.
 * 
 * Approach: Read source files and verify a11y patterns via regex.
 */

// From src/lib/shared/utils/ go up 4 levels to frontend/
const frontendDir = resolve(import.meta.dirname, '../../../..');

function readPage(relativePath: string): string {
  const fullPath = resolve(frontendDir, relativePath);
  if (!existsSync(fullPath)) {
    throw new Error(`File not found: ${fullPath}`);
  }
  return readFileSync(fullPath, 'utf-8');
}

// ═══════════════════════════════════════════════════════════════
// 1. BASE HTML
// ═══════════════════════════════════════════════════════════════
describe('Base HTML (app.html)', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/app.html'); });

  it('has lang attribute on html element', () => {
    expect(html).toMatch(/lang="[^"]+"/);
  });

  it('has viewport meta tag', () => {
    expect(html).toMatch(/name="viewport"/);
    expect(html).toMatch(/width=device-width/);
  });

  it('has charset meta tag', () => {
    expect(html).toMatch(/charset="utf-8"/);
  });

  it('does not use deprecated elements', () => {
    for (const el of ['<marquee', '<blink', '<font', '<center']) {
      expect(html.toLowerCase()).not.toContain(el);
    }
  });

  it('has body element', () => {
    expect(html).toMatch(/<body/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 2. LOGIN PAGE
// ═══════════════════════════════════════════════════════════════
describe('Login Page', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/routes/login/+page.svelte'); });

  it('uses semantic form element', () => {
    expect(html).toMatch(/<form/);
  });

  it('has proper input types', () => {
    expect(html).toMatch(/type="text"/);
    expect(html).toMatch(/type="password"/);
  });

  it('form button has type="submit"', () => {
    expect(html).toMatch(/type="submit"/);
  });

  it('inputs are required', () => {
    expect(html).toMatch(/required/);
  });

  it('has label elements for inputs', () => {
    expect(html).toMatch(/<label/);
  });

  it('inputs have placeholder text', () => {
    expect(html).toMatch(/placeholder="[^"]+"/);
  });

  it('has theme toggle button with aria-label', () => {
    expect(html).toMatch(/aria-label="Toggle theme"/);
  });

  it('has loading state with disabled button', () => {
    expect(html).toMatch(/disabled=\{loading\}/);
  });

  it('displays error messages', () => {
    expect(html).toMatch(/\{#if error\}/);
  });

  it('has focus styles on inputs', () => {
    expect(html).toMatch(/focus:ring/);
  });

  it('button has minimum touch target height', () => {
    expect(html).toMatch(/min-h-\[44px\]/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 3. LAYOUT
// ═══════════════════════════════════════════════════════════════
describe('Dashboard Layout', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/routes/+layout.svelte'); });

  it('uses semantic main element', () => {
    expect(html).toMatch(/<main/);
  });

  it('renders children content (Svelte 5)', () => {
    expect(html).toMatch(/children/);
  });

  it('imports Sidebar component', () => {
    expect(html).toMatch(/import Sidebar/);
  });

  it('imports Header component', () => {
    expect(html).toMatch(/import Header/);
  });

  it('handles auth state', () => {
    expect(html).toMatch(/isAuthenticated|isLoading/);
  });

  it('has login redirect logic', () => {
    expect(html).toMatch(/goto\('\/login'\)/);
  });

  it('manages WebSocket lifecycle', () => {
    expect(html).toMatch(/connectWebSocket|disconnectWebSocket/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 4. SIDEBAR
// ═══════════════════════════════════════════════════════════════
describe('Sidebar Navigation', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/components/Sidebar.svelte'); });

  it('uses semantic aside element', () => {
    expect(html).toMatch(/<aside/);
  });

  it('uses semantic nav element', () => {
    expect(html).toMatch(/<nav/);
  });

  it('has aria-label on navigation', () => {
    expect(html).toMatch(/aria-label="[^"]*"/);
  });

  it('has role="navigation" on aside', () => {
    expect(html).toMatch(/role="navigation"/);
  });

  it('nav items have links with href', () => {
    expect(html).toMatch(/href=\{item\.href\}/);
  });

  it('has active state indicator', () => {
    expect(html).toMatch(/isActive/);
  });

  it('mobile toggle button has aria-label', () => {
    expect(html).toMatch(/aria-label=.*Close menu|Open menu/);
  });

  it('navigation items have minimum touch target', () => {
    expect(html).toMatch(/min-h-\[44px\]/);
  });

  it('has hover effects', () => {
    expect(html).toMatch(/hover:/);
  });

  it('has focus styles', () => {
    expect(html).toMatch(/focus-visible:/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });

  it('has responsive breakpoint for tablet', () => {
    expect(html).toMatch(/md:/);
  });

  it('has responsive breakpoint for mobile', () => {
    expect(html).toMatch(/lg:hidden/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 5. SESSIONS PAGE
// ═══════════════════════════════════════════════════════════════
describe('Sessions Page', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/routes/sessions/+page.svelte'); });

  it('has search input', () => {
    expect(html).toMatch(/<input/);
    expect(html).toMatch(/placeholder="[^"]*[Ss]earch[^"]*"/);
  });

  it('has sort select', () => {
    expect(html).toMatch(/<select/);
  });

  it('has sort options', () => {
    expect(html).toMatch(/<option/);
  });

  it('input has minimum touch target', () => {
    expect(html).toMatch(/min-h-\[44px\]/);
  });

  it('has focus styles on inputs', () => {
    expect(html).toMatch(/focus:ring/);
  });

  it('uses SessionCard component', () => {
    expect(html).toMatch(/import SessionCard/);
  });

  it('uses PullToRefresh component', () => {
    expect(html).toMatch(/import PullToRefresh/);
  });

  it('handles loading state', () => {
    expect(html).toMatch(/loading/);
  });

  it('handles empty state', () => {
    expect(html).toMatch(/filteredSessions\.length === 0/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 6. SETTINGS PAGE
// ═══════════════════════════════════════════════════════════════
describe('Settings Page', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/routes/settings/+page.svelte'); });

  it('has proper heading', () => {
    expect(html).toMatch(/<h1/);
  });

  it('uses grid layout for sections', () => {
    expect(html).toMatch(/grid/);
  });

  it('displays config data', () => {
    expect(html).toMatch(/config/);
  });

  it('has external link with rel="noopener"', () => {
    expect(html).toMatch(/target="_blank"/);
    expect(html).toMatch(/rel="noopener"/);
  });

  it('handles loading state', () => {
    expect(html).toMatch(/loading/);
  });

  it('handles error state', () => {
    expect(html).toMatch(/error/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });

  it('has responsive classes', () => {
    expect(html).toMatch(/sm:|md:/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 7. STATSCARD COMPONENT
// ═══════════════════════════════════════════════════════════════
describe('StatsCard Component', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/components/StatsCard.svelte'); });

  it('uses button element (interactive)', () => {
    expect(html).toMatch(/<button/);
  });

  it('has haptic feedback', () => {
    expect(html).toMatch(/haptic/);
  });

  it('has hover effects', () => {
    expect(html).toMatch(/hover:/);
  });

  it('has active state', () => {
    expect(html).toMatch(/active:/);
  });

  it('has minimum touch target height', () => {
    expect(html).toMatch(/min-h-\[80px\]/);
  });

  it('has cursor pointer', () => {
    expect(html).toMatch(/cursor-pointer/);
  });

  it('has focus styles', () => {
    expect(html).toMatch(/focus-visible:/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });

  it('has responsive padding', () => {
    expect(html).toMatch(/sm:|md:/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 8. SESSIONCARD COMPONENT
// ═══════════════════════════════════════════════════════════════
describe('SessionCard Component', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/components/SessionCard.svelte'); });

  it('has haptic feedback', () => {
    expect(html).toMatch(/haptic/);
  });

  it('has swipe detection', () => {
    expect(html).toMatch(/swipe|createSwipeDetector/);
  });

  it('has hover effects', () => {
    expect(html).toMatch(/hover:/);
  });

  it('has focus styles', () => {
    expect(html).toMatch(/focus-visible:/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });

  it('has transition effects', () => {
    expect(html).toMatch(/transition/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 9. HEADER COMPONENT
// ═══════════════════════════════════════════════════════════════
describe('Header Component', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/components/Header.svelte'); });

  it('displays status indicator', () => {
    expect(html).toMatch(/online|offline/);
  });

  it('has haptic feedback', () => {
    expect(html).toMatch(/haptic/);
  });

  it('supports dark mode', () => {
    expect(html).toMatch(/dark:/);
  });

  it('has responsive classes', () => {
    expect(html).toMatch(/sm:|md:/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 10. THEME STORE
// ═══════════════════════════════════════════════════════════════
describe('Theme Store', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/stores/theme.ts'); });

  it('stores theme preference in localStorage', () => {
    expect(html).toMatch(/localStorage/);
  });

  it('supports system preference detection', () => {
    expect(html).toMatch(/prefers-color-scheme/);
  });

  it('has toggle function', () => {
    expect(html).toMatch(/toggle/);
  });

  it('has init function', () => {
    expect(html).toMatch(/init/);
  });

  it('supports dark, light, and system modes', () => {
    expect(html).toMatch(/dark|light|system/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 11. TOUCH UTILITIES
// ═══════════════════════════════════════════════════════════════
describe('Touch Utilities', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/shared/utils/touch.ts'); });

  it('has haptic feedback support', () => {
    expect(html).toMatch(/vibrate/);
  });

  it('creates swipe detector', () => {
    expect(html).toMatch(/createSwipeDetector/);
  });

  it('has destroy method for cleanup', () => {
    expect(html).toMatch(/destroy/);
  });

  it('registers touch event listeners', () => {
    expect(html).toMatch(/addEventListener|removeEventListener/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 12. API UTILITY
// ═══════════════════════════════════════════════════════════════
describe('API Utility', () => {
  let html: string;
  beforeAll(() => { html = readPage('src/lib/shared/utils/api.ts'); });

  it('uses fetch API', () => {
    expect(html).toMatch(/fetch\(/);
  });

  it('includes credentials for cookies', () => {
    expect(html).toMatch(/credentials.*include/);
  });

  it('handles errors gracefully', () => {
    expect(html).toMatch(/try|catch|error/);
  });

  it('provides meaningful error messages', () => {
    expect(html).toMatch(/Error|error|throw/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 13. RESPONSIVE DESIGN
// ═══════════════════════════════════════════════════════════════
describe('Responsive Design', () => {
  const components = [
    { file: 'src/routes/+page.svelte', name: 'Dashboard' },
    { file: 'src/routes/sessions/+page.svelte', name: 'Sessions' },
    { file: 'src/routes/settings/+page.svelte', name: 'Settings' },
  ];

  it('all pages use responsive classes', () => {
    for (const { file, name } of components) {
      const html = readPage(file);
      expect(html, `${name} should have responsive classes`).toMatch(/sm:|md:|lg:/);
    }
  });
});

// ═══════════════════════════════════════════════════════════════
// 14. DARK MODE SUPPORT
// ═══════════════════════════════════════════════════════════════
describe('Dark Mode Support', () => {
  const files = [
    'src/lib/components/StatsCard.svelte',
    'src/lib/components/SessionCard.svelte',
    'src/lib/components/Sidebar.svelte',
    'src/lib/components/Header.svelte',
  ];

  it('all components have dark mode classes', () => {
    for (const file of files) {
      const html = readPage(file);
      expect(html, `${file} should have dark: classes`).toMatch(/dark:/);
    }
  });
});

// ═══════════════════════════════════════════════════════════════
// 15. KEYBOARD NAVIGATION
// ═══════════════════════════════════════════════════════════════
describe('Keyboard Navigation', () => {
  it('interactive elements have focus styles', () => {
    const files = [
      'src/lib/components/StatsCard.svelte',
      'src/lib/components/Sidebar.svelte',
    ];
    for (const file of files) {
      const html = readPage(file);
      expect(html, `${file} should have focus: styles`).toMatch(/focus-visible:/);
    }
  });
});

// ═══════════════════════════════════════════════════════════════
// 16. TOUCH TARGETS (WCAG 2.5.8)
// ═══════════════════════════════════════════════════════════════
describe('Touch Targets', () => {
  it('navigation items have minimum 44px touch targets', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/min-h-\[44px\]/);
  });

  it('login buttons have minimum 44px touch targets', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/min-h-\[44px\]/);
  });

  it('session page inputs have minimum 44px touch targets', () => {
    const html = readPage('src/routes/sessions/+page.svelte');
    expect(html).toMatch(/min-h-\[44px\]/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 17. FORM ACCESSIBILITY
// ═══════════════════════════════════════════════════════════════
describe('Form Accessibility', () => {
  it('login form uses semantic form element', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/<form/);
  });

  it('login form has label elements', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/<label/);
  });

  it('inputs have required attribute', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/required/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 18. ARIA & SEMANTIC HTML
// ═══════════════════════════════════════════════════════════════
describe('ARIA & Semantic HTML', () => {
  it('sidebar navigation has aria-label', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/aria-label="[^"]*"/);
  });

  it('sidebar uses aside element', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/<aside/);
  });

  it('sidebar uses nav element', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/<nav/);
  });

  it('layout uses main element', () => {
    const html = readPage('src/routes/+layout.svelte');
    expect(html).toMatch(/<main/);
  });

  it('login theme toggle has aria-label', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/aria-label="Toggle theme"/);
  });

  it('sidebar mobile toggle has aria-label', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/aria-label=\{sidebarOpen/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 19. ERROR & LOADING STATES
// ═══════════════════════════════════════════════════════════════
describe('Error & Loading States', () => {
  it('login page handles errors', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/\{#if error\}/);
  });

  it('login page has loading state', () => {
    const html = readPage('src/routes/login/+page.svelte');
    expect(html).toMatch(/loading/);
    expect(html).toMatch(/disabled=\{loading\}/);
  });

  it('sessions page handles loading', () => {
    const html = readPage('src/routes/sessions/+page.svelte');
    expect(html).toMatch(/loading/);
  });

  it('sessions page handles empty state', () => {
    const html = readPage('src/routes/sessions/+page.svelte');
    expect(html).toMatch(/filteredSessions\.length === 0/);
  });

  it('settings page handles loading', () => {
    const html = readPage('src/routes/settings/+page.svelte');
    expect(html).toMatch(/loading/);
  });

  it('settings page handles errors', () => {
    const html = readPage('src/routes/settings/+page.svelte');
    expect(html).toMatch(/error/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 20. API SECURITY
// ═══════════════════════════════════════════════════════════════
describe('API Security', () => {
  it('fetch wrapper includes credentials', () => {
    const html = readPage('src/lib/shared/utils/api.ts');
    expect(html).toMatch(/credentials.*include/);
  });

  it('settings page uses credentials', () => {
    const html = readPage('src/routes/settings/+page.svelte');
    expect(html).toMatch(/credentials.*include/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 21. EXTERNAL LINKS
// ═══════════════════════════════════════════════════════════════
describe('External Links', () => {
  it('settings page GitHub link has rel="noopener"', () => {
    const html = readPage('src/routes/settings/+page.svelte');
    expect(html).toMatch(/target="_blank"/);
    expect(html).toMatch(/rel="noopener"/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 22. THEME PERSISTENCE
// ═══════════════════════════════════════════════════════════════
describe('Theme Persistence', () => {
  it('theme preference is saved to localStorage', () => {
    const html = readPage('src/lib/stores/theme.ts');
    expect(html).toMatch(/localStorage\.setItem/);
  });

  it('theme preference is loaded from localStorage', () => {
    const html = readPage('src/lib/stores/theme.ts');
    expect(html).toMatch(/localStorage\.getItem/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 23. HAPTIC FEEDBACK (Mobile a11y)
// ═══════════════════════════════════════════════════════════════
describe('Haptic Feedback', () => {
  it('StatsCard uses haptic feedback', () => {
    const html = readPage('src/lib/components/StatsCard.svelte');
    expect(html).toMatch(/haptic/);
  });

  it('SessionCard uses haptic feedback', () => {
    const html = readPage('src/lib/components/SessionCard.svelte');
    expect(html).toMatch(/haptic/);
  });

  it('Sidebar uses haptic feedback', () => {
    const html = readPage('src/lib/components/Sidebar.svelte');
    expect(html).toMatch(/haptic/);
  });
});

// ═══════════════════════════════════════════════════════════════
// 24. APP.CSS
// ═══════════════════════════════════════════════════════════════
describe('App CSS', () => {
  it('has Tailwind CSS import', () => {
    const html = readPage('src/app.css');
    expect(html).toMatch(/tailwindcss|@import/);
  });

  it('has dark mode class support', () => {
    const html = readPage('src/app.css');
    expect(html).toMatch(/dark/);
  });
});
