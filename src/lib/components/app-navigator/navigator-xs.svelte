<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';


	import {
        AlignJustify,
        Home,
        Search,
        Slash,
	} from '@lucide/svelte/icons';

	let {
        
		class: className
	}: {
        
		class?: string;
	} = $props();

    let windowWidthNotTooSmall = $state(typeof window !== 'undefined' && window.innerWidth >= 430);

	onMount(async () => {
        windowWidthNotTooSmall = typeof window !== 'undefined' && window.innerWidth >= 430;
        window.addEventListener('resize', () => {
            windowWidthNotTooSmall = typeof window !== 'undefined' && window.innerWidth >= 430;
        });
	});
</script>

<section
    data-tauri-drag-region
    class={cn('w-full mx-auto h-7 p-0.5 md:px-2 justify-center items-center gap-1', className, "flex sm:hidden bg-transparent border-none")}
>
    <Button
        variant="ghost"
        class={cn("h-7.5 w-fit px-2 text-foreground/75 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent",
            "border border-border bg-accent/20"
        )}
        href="/"
        title="Home / Navigation"
    >
        <div class="flex items-center gap-0.5">
            {#if windowWidthNotTooSmall}
                <Home class="size-4" />
                <Slash class="size-3 -rotate-12" />
                <AlignJustify class="size-4" />
            {:else}
                <AlignJustify class="size-4" />
            {/if}
        </div>
    </Button>

    
    {#if windowWidthNotTooSmall}
        <Button
            variant="ghost"
            class={cn("size-7.5 text-foreground/75 hover:!bg-accent hover:!text-accent-foreground/100 focus-visible:!bg-accent focus-visible:!ring-0 focus-visible:!ring-transparent",
                "border border-border bg-accent/20"
            )}
            title="Search (Ctrl+K)"
            onclick={() => {
                
            }}
        >
            <Search class="size-4" />
        </Button>
    {/if}
</section>