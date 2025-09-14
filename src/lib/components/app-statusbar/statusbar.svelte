<script lang="ts">
	import "./style.css";

	import { invoke } from "@tauri-apps/api/core";

	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from "$lib/components/ui/select/index.js";

	import {
		Cable,
		Cpu,
		ChevronsLeftRightEllipsis,
		LoaderCircle,
		RotateCcw,
		RouteOff,
	} from '@lucide/svelte/icons';

	import { goto } from "$app/navigation";
    import { get } from "svelte/store";

	let {
		class: className
	}: {
		class?: string;
	} = $props();

	onMount(() => {
	});
</script>


<section class={cn(
		'statusbar-override isolate',
		'bg-background/90 pointer-events-auto fixed left-0 bottom-0 z-[99999] border-t border-primary/15',
		className,
		'w-full h-6'
	)}
>
    <div class="w-full h-full flex items-center justify-between gap-2">
		<section data-statusbar-left class="w-fit h-full flex items-center justify-center gap-1.5">
			
		</section>

		<section data-statusbar-center class="w-fit h-full flex items-center justify-center gap-1.5">
			
		</section>

		<section data-statusbar-right class="w-fit h-full flex items-center justify-center gap-1.5">
			
		</section>

	</div>
</section>