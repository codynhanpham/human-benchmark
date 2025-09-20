<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    import { Button } from "$lib/components/ui/button/index.js";

    import { LoaderCircle } from "@lucide/svelte/icons";

    import { page } from "$app/state";
    import { onMount } from "svelte";

    let mousePosition = $state({ x: 0, y: 0 });
    let mouseInterval: number;

    let isRunning = $state(false);

    onMount(() => {
        mouseInterval = setInterval(() => {
            invoke("get_mouse_position").then((pos) => {
                mousePosition = { x: (pos as number[])[0], y: (pos as number[])[1] };
            });
        }, 10);
        return () => clearInterval(mouseInterval);
    });

</script>

<main class="w-full h-full min-h-fit space-y-2 p-2">
    <Button
        id="start-ocr"
        variant="outline"
        class="w-full"
        onclick={async () => {
            isRunning = true;
            await invoke("start_number_memory").catch((_) => {
                isRunning = false;
            });
            isRunning = false;
        }}
        disabled={isRunning}
    >
        {#if isRunning}
            <LoaderCircle class="animate-spin mr-1" />
            <span>
                [Please don't move the cursor!]
            </span>
        {:else}
            <span>
                Start Number Memory Test
            </span>
        {/if}
    </Button>

    <p class="text-center">({mousePosition.x}, {mousePosition.y})</p>
</main>
