<script lang="ts">
	import { Icon } from "@steeze-ui/svelte-icon";
	import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton";
	import { LightBulb, Signal } from "@steeze-ui/heroicons";
	import { page } from "$app/stores";
	import { goto } from "$app/navigation";

	$: valueSingle = routes.find((route) =>
		route.path === "/settings"
			? $page.url.pathname === "/settings"
			: $page.url.pathname.includes(route.path)
	)?.path;

	const routes = [
		{
			name: "About",
			icon: LightBulb,
			path: "/settings",
		},
		{
			name: "Sensors",
			icon: Signal,
			path: "/settings/sensors",
		},
	];
</script>

<div class="flex w-full h-full">
	<ListBox class="transition-all w-52 mr-8" regionDefault="" active="variant-filled-primary">
		{#each routes as { name, icon, path }}
			<ListBoxItem
				bind:group={valueSingle}
				name="medium"
				value={path}
				class="transition-all"
				on:click={() => goto(path)}
			>
				<svelte:fragment slot="lead">
					<Icon src={icon} class="w-5 h-5 transition-all" />
				</svelte:fragment>
				{name}
			</ListBoxItem>
		{/each}
	</ListBox>
	<div class="w-full">
		<slot />
	</div>
</div>
