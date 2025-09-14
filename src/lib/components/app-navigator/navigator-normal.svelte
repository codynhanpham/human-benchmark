<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount, tick } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';

    import { goto } from '$app/navigation';
    import { page } from '$app/state';

    import { NavigationRoutes } from './navigatorData.svelte';

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
    class={cn('mx-auto w-fit h-7 justify-center items-center gap-1 hidden sm:flex')}
>
    <section
        data-tauri-drag-region
        class={cn('flex justify-center items-center gap-0.5 lg:gap-1')}
    >
        <Button
            id=""
            variant="ghost"
            class={cn("size-6.5 text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent",
            )}
            onclick={async () => {
                window.history.back();
            }}
            title="Back"
        >
            <ChevronLeft class="size-4" />
        </Button>
        <Button
            id=""
            variant="ghost"
            class={cn("size-6.5 text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent",
            )}
            onclick={async () => {
                window.history.forward();
            }}
            title="Forward"
        >
            <ChevronRight class="size-4" />
        </Button>
        <Button
            id=""
            variant="ghost"
            class={cn("size-6.5 text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent",

            )}
            onclick={async () => {
                location.reload();
            }}
            title="Reload Page"
        >
            <RotateCw class="size-4" />
        </Button>
    </section>


    <section
        data-tauri-drag-region
        class={cn('w-full h-7 p-0.5 !px-[calc(var(--spacing)*0.25)] flex justify-center items-center gap-1 cursor-default ml-2 md:ml-3', className)}
    >
        {#each NavigationRoutes as route}
            <Button
                variant="ghost"
                class={cn("size-6.5 md:w-7.5 lg:w-8 text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 transition-colors duration-350 ease-out",
                    (route.url === '/' && page.url.pathname === '/') || (route.url !== '/' && page.url.pathname.startsWith(route.url)) ? 'bg-primary text-primary-foreground hover:!bg-primary hover:!text-primary-foreground focus-visible:!bg-primary focus-visible:!text-primary-foreground' : ''
                )}
                href={route.url}
                title={route.name}
            >
                <route.icon class="size-4" />
            </Button>
        {/each}
    </section>

    <Button
        variant="ghost"
        class={cn("ml-0.5 size-7.5 text-foreground/70 hover:!bg-accent hover:!text-accent-foreground/100 border border-primary/20 rounded-lg",
        )}
        title="Search (Ctrl+K)"
        onclick={() => {
            
        }}
    >
        <Search class="size-4" />
    </Button>
</div>
