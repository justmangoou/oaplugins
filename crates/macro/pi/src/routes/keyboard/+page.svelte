<script lang="ts">
    import { actionSettings } from "@openaction/svelte-pi";

    let selectedType = $derived(
        $actionSettings.type ?? "Single",
    );

    let selectedButton = $derived(
        $actionSettings.button ?? "Left",
    );

    function updateType(event: Event) {
        const type = (event.target as HTMLSelectElement).value;
        $actionSettings = { ...$actionSettings, type };
    }

    function updateButton(event: Event) {
        const button = (event.target as HTMLSelectElement).value;
        $actionSettings = { ...$actionSettings, button };
    }
</script>

<div class="space-y-4 text-neutral-200">
    <div class="grid grid-cols-[250px_1fr] items-center">
        <label for="type" class="text-sm">Click Type</label>
        <div class="select-wrapper">
            <select
                id="type"
                value={selectedType}
                onchange={updateType}
                class="w-full"
            >
                <option value="Single">Single</option>
                <option value="Double">Double</option>
                <option value="Hold">Hold</option>
            </select>
        </div>
    </div>

    <div class="grid grid-cols-[250px_1fr] items-center">
        <label for="button" class="text-sm">Text</label>
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
            </select>
        </div>
    </div>
</div>
