<script lang="ts">
    import { actionSettings } from "@openaction/svelte-pi";

    let selectedClickType = $derived($actionSettings.click_type ?? "Single");
    let selectedButton = $derived($actionSettings.button ?? "Left");
    let interval = $derived($actionSettings.interval ?? 100);

    function updateClickType(event: Event) {
        const click_type = (event.target as HTMLSelectElement).value;
        $actionSettings = { ...$actionSettings, click_type };
    }

    function updateButton(event: Event) {
        const button = (event.target as HTMLSelectElement).value;
        $actionSettings = { ...$actionSettings, button };
    }

    function updateInterval(event: Event) {
        const interval = parseInt((event.target as HTMLInputElement).value);
        $actionSettings = { ...$actionSettings, interval };
    }
</script>

<div class="space-y-4 text-neutral-200">
    <div class="grid grid-cols-[250px_1fr] items-center">
        <label for="click-type" class="text-sm">Click Type</label>
        <div class="select-wrapper">
            <select
                id="click-type"
                value={selectedClickType}
                onchange={updateClickType}
                class="w-full"
            >
                <option value="Single">Single</option>
                <option value="Double">Double</option>
                <option value="Hold">Hold</option>
            </select>
        </div>
    </div>

    <div class="grid grid-cols-[250px_1fr] items-center">
        <label for="button" class="text-sm">Mouse Button</label>
        <div class="select-wrapper">
            <select
                id="button"
                value={selectedButton}
                onchange={updateButton}
                class="w-full"
            >
                <option value="Left">Left</option>
                <option value="Right">Right</option>
                <option value="Middle">Middle</option>
                <option value="Back">Back</option>
                <option value="Forward">Forward</option>
            </select>
        </div>
    </div>

    <div class="grid grid-cols-[250px_1fr] items-center">
        <label for="interval" class="text-sm">Interval</label>

        <div class="flex items-center gap-2">
            <input
                id="interval"
                type="number"
                min="1"
                bind:value={interval}
                onchange={updateInterval}
                class="w-full rounded-lg border border-neutral-600 bg-neutral-700 px-3 py-2 text-sm text-neutral-300 outline-none"
            />

            <span class="text-sm text-neutral-400 whitespace-nowrap">
                ms
            </span>
        </div>
    </div>

</div>
