<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount, tick } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
    import Separator from '$lib/components/ui/separator/separator.svelte';
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";

    import { goto } from '$app/navigation';
    import { page } from '$app/state';

    import { NavigationRoutes, asNavigationRoute } from './navigatorData.svelte';

	import {
        ChevronLeft,
        ChevronRight,
        CirclePlus,
        History,
        Home,
        LibraryBig,
        RotateCw,
        Search,
        Settings,
	} from '@lucide/svelte/icons';

    import { type IconNode, Icon } from "@lucide/svelte";


	let {
        
		class: className
	}: {
        
		class?: string;
	} = $props();


    onMount(() => {
        
    });
</script>

<div
    data-tauri-drag-region
    class={cn('mx-auto w-fit h-7.5 justify-center items-center gap-1 hidden min-[490px]:flex')}
>
    <section
        data-tauri-drag-region
        class={cn('w-full h-7.5 p-0.5 !px-[calc(var(--spacing)*0.25)] flex justify-center items-center gap-1 cursor-default', className)}
    >
        {#each NavigationRoutes as routeData}
            {#if routeData.name === "separator"}
                <Separator orientation="vertical" class="!h-5 mx-0.5 bg-accent-foreground/40" />
            {:else}
                {@const route = asNavigationRoute(routeData)}
                <Tooltip.Provider>
                <Tooltip.Root
                    delayDuration={10}
                >
                    <Tooltip.Trigger
                        role="button"
                        class={cn(
                            buttonVariants({ variant: "ghost" }),
                            "size-7 md:w-7.5 lg:w-8 cursor-pointer text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 transition-colors duration-350 ease-out",
                            (route.url === '/' && page.url.pathname === '/') || (route.url !== '/' && page.url.pathname.startsWith(route.url)) ? 'bg-primary text-primary-foreground hover:!bg-primary hover:!text-primary-foreground focus-visible:!bg-primary focus-visible:!text-primary-foreground' : ''
                        )}
                        onclick={() => {
                            goto(route.url);
                        }}
                    >
                        {#if typeof route.icon === 'function'}
                            <route.icon class="size-4" {...route.iconAttributes} />
                        {:else if typeof route.icon === 'object'}
                            <Icon iconNode={route.icon as IconNode} class="size-4" {...route.iconAttributes} />
                        {/if}
                    </Tooltip.Trigger>
                    <Tooltip.Content class="pointer-events-none select-none">
                        <p class="pointer-events-none select-none">{route.name}</p>
                    </Tooltip.Content>
                </Tooltip.Root>
                </Tooltip.Provider>
            {/if}
        {/each}
    </section>
</div>
