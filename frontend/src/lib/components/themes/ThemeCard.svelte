<script lang="ts">
	import type { Theme } from "$lib/stores/installedThemes";

	export let theme: Theme;
	$: ({ name, author, image_src, fs_name } = theme);
	export let isLink = true;
	export let dimensions = "h-58 w-58";
	export let containerDimensions = "w-64";
</script>

<svelte:element
	this={isLink ? "a" : "div"}
	class={"block card card-hover p-4 relative overflow-hidden group dur-longer " +
		containerDimensions}
	href={isLink ? `/theme?fsName=${encodeURIComponent(fs_name)}` : undefined}
>
	<img
		class={"object-cover rounded-lg aspect-square absolute top-0 left-0 blur-lg brightness-50 transition-all dur-longer " +
			dimensions}
		src={image_src}
		alt={name}
	/>
	<div class="flex flex-col relative overflow-hidden">
		<img
			class={"overflow-hidden object-cover rounded-lg aspect-square group-hover:blur-md group-hover:brightness-75 transition-all dur-longer" +
				dimensions}
			src={image_src}
			alt={name}
		/>
	</div>
	<div
		class="transition-all dur-longer flex flex-col absolute top-0 left-0 w-full h-full opacity-0 group-hover:opacity-100 items-center align-center justify-center"
	>
		<h2 class="text-2xl font-bold">{name}</h2>
		<p class="text-gray-500">{author}</p>
	</div>
</svelte:element>

<style>
	.dur-longer {
		transition-duration: 400ms;
	}
</style>
