<script lang="ts">
    import { actionSettings } from "@openaction/svelte-pi";
    import ApplicationSettings from "$lib/ApplicationSettings.svelte";

    const MIN_STEP_SIZE = 1;
    const MAX_STEP_SIZE = 10;
    const DEFAULT_STEP_SIZE = 5;

    let currentStepSize = $derived(
        $actionSettings.step_size ?? DEFAULT_STEP_SIZE,
    );

    function updateStepSize(event: Event) {
        const step_size = parseInt((event.target as HTMLInputElement).value);
        $actionSettings = { ...$actionSettings, step_size };
    }
</script>

<div class="space-y-4 text-neutral-200">
    <div class="settings-grid">
        <label for="stepSize" class="text-sm">Volume Step Size</label>
        <div class="flex flex-row items-center space-x-4">
            <input
                id="stepSize"
                type="range"
                min={MIN_STEP_SIZE}
                max={MAX_STEP_SIZE}
                value={currentStepSize}
                oninput={updateStepSize}
                class="h-1.5 w-full cursor-pointer"
            />
            <span>{currentStepSize}%</span>
        </div>
    </div>
</div>

<hr class="my-4 border-neutral-700" />

<ApplicationSettings />
