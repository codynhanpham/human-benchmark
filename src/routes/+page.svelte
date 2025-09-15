<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    import { Button } from "$lib/components/ui/button/index.js";

    import { History } from "@lucide/svelte/icons";

    import { page } from "$app/state";


</script>

<main class="w-full h-full min-h-fit space-y-2 p-2">
    <Button
        id="start-ocr"
        variant="outline"
        class="w-full"
        onclick={async () => {
            await openUrl("https://humanbenchmark.com/")
        }}
    >
        Human Benchmark Website
    </Button>
</main>
