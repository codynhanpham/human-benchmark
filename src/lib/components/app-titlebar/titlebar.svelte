<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';

	import {
		Copy,
		Minus as IconMinus,
		Pin,
		Square,
		X as IconX,
	} from '@lucide/svelte/icons';

	let {
		class: className
	}: {
		class?: string;
	} = $props();

	let isMaximized = $state(false);
	let isAlwaysOnTop = $state(false);

	getCurrentWindow().onResized(async () => {
		isMaximized = await getCurrentWindow().isMaximized();
	});

	getCurrentWindow().isAlwaysOnTop().then((value) => {
		isAlwaysOnTop = value;
	});

	onMount(async () => {
		isMaximized = await getCurrentWindow().isMaximized();
	});
</script>

<div
	data-tauri-drag-region
	class={cn(
		'bg-accent/45 backdrop-blur-2xl pointer-events-auto fixed left-0 right-0 top-0 z-[999999] isolate h-fit border-b hover:cursor-grab active:cursor-grabbing',
		className,
		'!max-h-fit'
	)}
>
	<div data-tauri-drag-region class="flex h-fit w-full items-center justify-between gap-8">
		<div data-tauri-drag-region class="h-fit w-full max-w-fit">
			
		</div>

		<div data-tauri-drag-region class="flex h-fit min-w-max items-center gap-0.5">
			<Button
				id="titlebar-minimize"
				variant="ghost"
				class={cn("size-6.5 text-muted-foreground cursor-pointer hover:!bg-accent hover:!text-accent-foreground",
					isAlwaysOnTop ? 'bg-accent' : ''
				)}
				onclick={async () => {
					if (await getCurrentWindow().isAlwaysOnTop()) {
						await getCurrentWindow().setAlwaysOnTop(false);
						isAlwaysOnTop = false;
					} else {
						await getCurrentWindow().setAlwaysOnTop(true);
						isAlwaysOnTop = true;
					}
				}}
				title={isAlwaysOnTop ? 'Unpin from top' : 'Pin to top'}
			>
				<!-- <IconMinus aria-label="minimize" class="size-5" strokeWidth="2" /> -->
				{#if isAlwaysOnTop}
					<Pin aria-label="unpin" class="size-4 text-foreground" strokeWidth="2" />
				{:else}
					<Pin aria-label="pin" class="size-4 text-muted-foreground/80" strokeWidth="2" />
				{/if}
			</Button>

			<div class="w-2"></div>

			<Button
				id="titlebar-minimize"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-accent hover:!text-accent-foreground focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().minimize();
				}}
			>
				<IconMinus aria-label="minimize" class="size-5" strokeWidth="2" />
			</Button>
			<Button
				id="titlebar-restore"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-accent hover:!text-accent-foreground focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().toggleMaximize();
				}}
			>
				{#if isMaximized}
					<Copy aria-label="maximize" class="size-3.5 scale-x-[-1]" />
				{:else}
					<Square aria-label="restore" class="size-3.5" strokeWidth="2" />
				{/if}
			</Button>

			<Button
				id="titlebar-close"
				variant="ghost"
				class="h-8 w-10 text-muted-foreground hover:!bg-destructive hover:!text-foreground focus-visible:!bg-destructive focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					getCurrentWindow().close();
				}}
			>
				<IconX aria-label="close" class="size-5" strokeWidth="1.5" />
			</Button>
		</div>
	</div>
</div>
