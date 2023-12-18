<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { RadioGroup, RadioItem } from "@skeletonlabs/skeleton";

	const goToAbleRoutes = [
		{
			title: "Themes",
			path: "/",
		},
		{
			title: "Now Playing",
			path: "/now-playing",
		},
		{
			title: "Device",
			path: "/device",
		},
		{
			title: "Settings",
			path: "/settings",
		},
	];

	$: tabSet = goToAbleRoutes.find((route) =>
		route.path === "/"
			? $page.url.pathname === "/" || $page.url.pathname.startsWith("/theme")
			: $page.url.pathname.includes(route.path)
	)?.path;
</script>

<aside class="flex align-center items-center">
	<RadioGroup active="variant-filled-primary" hover="hover:variant-soft-primary" class="ml-2">
		{#each goToAbleRoutes as route}
			<RadioItem
				bind:group={tabSet}
				name="justify"
				value={route.path}
				class="transition-all"
				on:click={() => goto(route.path)}
			>
				{route.title}
			</RadioItem>
		{/each}
	</RadioGroup>
</aside>
