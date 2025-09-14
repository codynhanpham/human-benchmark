<script lang="ts" module>

	restoreStateCurrent(StateFlags.ALL);
</script>

<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";

	let { children } = $props();


	import { getCurrentWindow } from '@tauri-apps/api/window';
	import {
		restoreStateCurrent,
		saveWindowState,
		StateFlags
	} from '@tauri-apps/plugin-window-state';


	import { onMount, onDestroy } from 'svelte';
    import { onNavigate } from '$app/navigation';
    import { cn } from '$lib/utils';


	import { TitleBar } from '$lib/components/app-titlebar';
	import { StatusBar } from '$lib/components/app-statusbar';


	onMount(() => {



		// After the app is loaded, show the app window
		getCurrentWindow().show();

		// Save the window state after the app is loaded
		saveWindowState(StateFlags.ALL);
		// Before reloading the page (with Ctrl R, F5, Ctrl F5, etc.), save the window state
		window.addEventListener('beforeunload', () => {
			saveWindowState(StateFlags.ALL);
		});
	});

    onDestroy(async () => {
        await saveWindowState(StateFlags.ALL);
    });
</script>

<ModeWatcher />


<div class="fixed top-0 left-0 w-full h-fit z-[999999] isolate">
    <TitleBar />
</div>


<div class={cn("fixed top-8 left-0 w-full h-full overflow-auto",
        "max-h-[calc(100%-(var(--spacing)*8)-(var(--spacing)*6))]"
    )}
>
    <div class={cn(
        "relative h-full",
    )}>
        {@render children?.()}
    </div>
</div>

<StatusBar />