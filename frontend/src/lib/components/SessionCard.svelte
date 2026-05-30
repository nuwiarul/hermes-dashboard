<script lang="ts">
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
        return ended_at ? 'bg-gray-100 text-gray-600' : 'bg-green-100 text-green-700';
    }

    function getStatus(ended_at: number | null): string {
        return ended_at ? 'Ended' : 'Active';
    }
</script>

<div class="bg-white rounded-xl shadow-sm p-5 hover:shadow-md transition-all cursor-pointer border border-gray-100 hover:border-blue-200">
    <div class="flex justify-between items-start gap-4">
        <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-2">
                <span class="text-lg">{getSourceIcon(session.source)}</span>
                <h3 class="font-semibold text-lg text-gray-900 truncate">
                    {session.title || 'Untitled Session'}
                </h3>
            </div>
            
            <div class="flex flex-wrap items-center gap-3 text-sm text-gray-500">
                <span class="flex items-center gap-1">
                    <span class="capitalize">{session.source || 'unknown'}</span>
                </span>
                <span class="text-gray-300">•</span>
                <span class="flex items-center gap-1">
                    💬 {session.message_count || 0} messages
                </span>
                <span class="text-gray-300">•</span>
                <span class="flex items-center gap-1">
                    🤖 {session.model || 'unknown'}
                </span>
            </div>
        </div>
        
        <div class="flex flex-col items-end gap-2">
            <span class="text-xs px-2 py-1 rounded-full font-medium {getStatusColor(session.ended_at)}">
                {getStatus(session.ended_at)}
            </span>
            <span class="text-xs text-gray-400">
                {formatDate(session.started_at)}
            </span>
        </div>
    </div>
    
    <div class="mt-3 pt-3 border-t border-gray-100">
        <p class="text-xs text-gray-400 font-mono truncate">ID: {session.id}</p>
    </div>
</div>
