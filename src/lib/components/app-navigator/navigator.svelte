<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils.js';

	import { Button } from '$lib/components/ui/button/index.js';
    
    import NavigatorXS from "./navigator-xs.svelte";
    import NavigatorMD from "./navigator-normal.svelte";


	import {
        Home,
        Settings,
	} from '@lucide/svelte/icons';

	let {
        
		class: className,
        ...restProps
	}: {
		class?: string;
	} = $props();

	onMount(async () => {
	});
</script>

<NavigatorXS class={className} {...restProps} />
<NavigatorMD class={className} {...restProps} />