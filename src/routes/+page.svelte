<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    import { Button } from "$lib/components/ui/button/index.js";

    import { History } from "@lucide/svelte/icons";

    import { page } from "$app/state";
    import { onMount } from "svelte";

    let mousePosition = $state({ x: 0, y: 0 });
    let mouseInterval: number;

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
            const arena = await invoke("detect_play_arena");
            console.log("Detected Play Arena:", arena);
        }}
    >
        Detect Play Arena
    </Button>

    <p class="text-center">({mousePosition.x}, {mousePosition.y})</p>
</main>
