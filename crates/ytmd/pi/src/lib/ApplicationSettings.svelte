<script lang="ts">
    import { globalSettings, openUrl } from "@openaction/svelte-pi";

    let host = "127.0.0.1";
    let port = 9863;
    let loading = false;

    $: {
        if ($globalSettings.host != undefined) {
            host = $globalSettings.host;
        }
        if ($globalSettings.port != undefined) {
            port = $globalSettings.port;
        }
    }

    const handleSave = async (e: Event) => {
        $globalSettings = {
            ...$globalSettings,
            client_settings: {
              host,
              port,
              token: null,
            }
        };
    };

    const handleHostChange = (e: Event) => {
        const input = e.target as HTMLInputElement;
        host = input.value;
    };

    const handlePortChange = (e: Event) => {
        const input = e.target as HTMLInputElement;
        port = parseInt(input.value) || 9863;
    };
</script>

{#if $globalSettings.error}
    <div
        class="mb-3 rounded-lg border border-red-700 bg-red-900/30 p-2 text-xs text-red-300"
    >
        <strong class="font-semibold">Error:</strong>
        {$globalSettings.error}
    </div>
{:else if $globalSettings.token && !loading}
    <div
        class="mb-3 rounded-lg border border-green-700 bg-green-900/30 p-2 text-xs text-green-300"
    >
        ✓ Connected to YoutubeMusic Desktop App
    </div>
{/if}

<div class="space-y-4 text-neutral-200">
    <div class="settings-grid">
        <label for="host" class="text-sm">Host</label>
        <div class="input-wrapper">
            <input
                id="host"
                type="text"
                class="w-full"
                value={host}
                onchange={handleHostChange}
                placeholder="e.g., 127.0.0.1"
            />
        </div>
    </div>

    <div class="settings-grid">
        <label for="port" class="text-sm">Port</label>
        <div class="input-wrapper">
            <input
                id="port"
                type="number"
                class="w-full"
                value={port}
                onchange={handlePortChange}
                placeholder="e.g., 9863"
            />
        </div>
    </div>

    <div class="gap-2 pt-2 font-medium">
        <button
            onclick={handleSave}
            disabled={loading}
            class="w-full rounded transition disabled:bg-gray-800"
        >
            {loading ? "Authenticating..." : "Authenticate"}
        </button>
    </div>
</div>
