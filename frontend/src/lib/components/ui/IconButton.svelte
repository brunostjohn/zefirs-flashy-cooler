<script lang="ts">
	import { Icon, type IconSource } from "@steeze-ui/svelte-icon";

	export let src: IconSource;
	export let iconClass: string | undefined = undefined;
	export let btnClass: string | undefined = undefined;
</script>

<button
	type="button"
	class={btnClass ?? "btn variant-filled"}
	on:click
	on:keydown
	on:keyup
	on:keypress
>
	<Icon class={iconClass ?? "mr-2 w-6 h-6"} {src} />
	<slot />
</button>
