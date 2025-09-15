<script lang="ts">
	import "./style.css";

	import { invoke } from "@tauri-apps/api/core";
	import { openUrl } from "@tauri-apps/plugin-opener";

	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { cn } from '$lib/utils.js';

	import { toggleMode } from "mode-watcher";

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from "$lib/components/ui/select/index.js";
	import { Separator } from "$lib/components/ui/separator";

	import {
		ExternalLink,
		Github,
		Moon,
		Sun,
	} from '@lucide/svelte/icons';

	import { NavigationRoutes, asNavigationRoute } from '../app-navigator/navigatorData.svelte';

	import { goto } from "$app/navigation";
	import { page } from "$app/state";
	import { onMount } from 'svelte';

	let {
		class: className
	}: {
		class?: string;
	} = $props();

	let currentCorrespondingWebsite = $derived.by(() => {
		const currentPath = page.url.pathname;
		
		// Check if route has url, and that the url matches the current path, then return the ogWebsite
		const route = NavigationRoutes.find(r => ('url' in r) && r.url === currentPath);
		if (route && 'ogWebsite' in route) {
			return {
				name: route.name,
				ogWebsite: route.ogWebsite ? route.ogWebsite : "https://humanbenchmark.com"
			}
		}
		return {
			name: "Home",
			ogWebsite: "https://humanbenchmark.com"
		};
	});

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
		<section data-statusbar-left class="w-fit h-full flex items-center justify-center gap-0">
			<Button onclick={toggleMode} variant="ghost" class="w-8 text-foreground/75 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent" title="Toggle theme">
				<Sun
					class="size-4 rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
				/>
				<Moon
					class="absolute size-4 rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
				/>
				<span class="sr-only">Toggle theme</span>
			</Button>

			<Separator orientation="vertical" class="h-4" />

			<Button
				variant="ghost"
				class="h-5.5 px-1.5 text-foreground/75 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					await openUrl("https://github.com/codynhanpham/human-benchmark");
				}}
				title="View this project on GitHub"
			>
				<div class="flex items-center gap-1">
					<Github class="size-4" />
				</div>
			</Button>
		</section>

		<section data-statusbar-center class="w-fit h-full flex items-center justify-center gap-1.5">
			
		</section>

		<section data-statusbar-right class="w-fit h-full flex items-center justify-center gap-1.5">
			<Button
				variant="ghost"
				class="h-5.5 px-1.5 text-foreground/75 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent"
				onclick={async () => {
					await openUrl(currentCorrespondingWebsite.ogWebsite);
				}}
				title={`Open${(currentCorrespondingWebsite.name === "Home") ? "" : ` ${currentCorrespondingWebsite.name} on`} Human Benchmark website`}
			>
				<div class="flex items-center gap-1">
					<span class="text-xs font-normal mx-0.5">
						<span class={cn("hidden sm:inline", (currentCorrespondingWebsite.name === "Home") ? "!inline" : "")}>Human Benchmark</span>
						<span class={cn("hidden sm:inline", (currentCorrespondingWebsite.name === "Home") ? "!hidden" : "")}> - </span>
						{(currentCorrespondingWebsite.name !== "Home") ? currentCorrespondingWebsite.name : ""}
					</span>
					<ExternalLink class="size-3.5 mb-0.5" />
				</div>
			</Button>
		</section>

	</div>
</section>