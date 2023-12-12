<script lang="ts">
	import { page } from "$app/stores";
	import { TabAnchor, TabGroup } from "@skeletonlabs/skeleton";

	const goToAbleRoutes = [
		{
			title: "Themes",
			path: "/",
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

	let tabSet: string = "/";
</script>

<aside>
	<TabGroup active="border-b-2 border-primary-700" rounded="rounded-none" border="border-b-0">
		{#each goToAbleRoutes as route}
			<TabAnchor
				bind:group={tabSet}
				href={route.path}
				class="font-serif transition-all"
				value={route.path}
				name={route.title}
				selected={$page.url.pathname.startsWith(route.path)}
			>
				{route.title}
			</TabAnchor>
		{/each}
	</TabGroup>
</aside>
