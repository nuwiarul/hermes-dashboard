import { writable } from 'svelte/store';
import { browser } from '$app/environment';

interface Status {
    online: boolean;
    total_sessions: number;
    total_messages: number;
    timestamp: number;
}

export const status = writable<Status>({
    online: false,
    total_sessions: 0,
    total_messages: 0,
    timestamp: 0
});

export const stats = writable<any>({
    total_sessions: 0,
    total_messages: 0,
    total_tool_calls: 0,
    estimated_cost_usd: 0,
    sessions_today: 0,
    messages_today: 0,
    active_sources: []
});

let ws: WebSocket | null = null;
let reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
let statsInterval: ReturnType<typeof setInterval> | null = null;

export function connectWebSocket() {
    if (!browser || ws) return;
    
    const wsUrl = `wss://api-hermes.vinrul.my.id/ws`;
    
    console.log('Connecting to WebSocket:', wsUrl);
    ws = new WebSocket(wsUrl);
    
    ws.onopen = () => {
        console.log('WebSocket connected');
        status.update(s => ({ ...s, online: true }));
        
        // Fetch initial stats via REST
        fetchStats();
        
        // Also poll stats every 10 seconds as backup
        statsInterval = setInterval(fetchStats, 10000);
    };
    
    ws.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            if (data.type === 'status') {
                // Update status
                status.set({
                    online: data.online,
                    total_sessions: data.total_sessions || 0,
                    total_messages: data.total_messages || 0,
                    timestamp: data.timestamp || 0
                });
                
                // Update stats from WebSocket
                stats.update(s => ({
                    ...s,
                    total_sessions: data.total_sessions || s.total_sessions,
                    total_messages: data.total_messages || s.total_messages
                }));
            }
        } catch (e) {
            console.error('Failed to parse WebSocket message:', e);
        }
    };
    
    ws.onclose = () => {
        console.log('WebSocket disconnected');
        status.update(s => ({ ...s, online: false }));
        ws = null;
        
        if (statsInterval) {
            clearInterval(statsInterval);
            statsInterval = null;
        }
        
        // Reconnect after 3 seconds
        reconnectTimeout = setTimeout(connectWebSocket, 3000);
    };
    
    ws.onerror = (error) => {
        console.error('WebSocket error:', error);
    };
}

async function fetchStats() {
    try {
        const baseUrl = import.meta.env.VITE_API_BASE_URL;
        const res = await fetch(`${baseUrl}/api/stats`);
        if (res.ok) {
            const data = await res.json();
            stats.set(data);
        }
    } catch (e) {
        console.error('Failed to fetch stats:', e);
    }
}

export function disconnectWebSocket() {
    if (reconnectTimeout) {
        clearTimeout(reconnectTimeout);
        reconnectTimeout = null;
    }
    if (statsInterval) {
        clearInterval(statsInterval);
        statsInterval = null;
    }
    if (ws) {
        ws.close();
        ws = null;
    }
}

export function sendPing() {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ type: 'ping' }));
    }
}
