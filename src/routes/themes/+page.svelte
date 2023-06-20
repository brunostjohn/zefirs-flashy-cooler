<script lang="ts">
	import { fade } from "svelte/transition";
	import type { Theme } from "./../../helpers/themeTools";
	import { onMount } from "svelte";
	import FeaturedThemes from "../../components/FeaturedThemes.svelte";
	import NavigationCircle from "../../components/NavigationCircle.svelte";
	import ThemeCarousel from "../../components/ThemeCarousel.svelte";
	import { getAllThemes, getFeaturedThemes } from "../../helpers/ghApi";
	import { getThemes } from "../../helpers/themeTools";
	import { DoubleBounce, Stretch } from "svelte-loading-spinners";

	let featuredThemes: Theme[] = [];
	let myThemes: Theme[] = [];
	let allThemes: Theme[] = [];
	let loadingFeatured = true;
	let loadingAll = true;
	let loadingMy = true;

	onMount(async () => {
		try {
			myThemes = await getThemes("most_used");
		} catch (e) {
			console.log(e);
		}
		loadingMy = false;
		try {
			featuredThemes = await getFeaturedThemes();
		} catch (e) {
			console.log(e);
		}
		loadingFeatured = false;
		try {
			allThemes = await getAllThemes(10);
		} catch (e) {
			console.log(e);
		}
		loadingAll = false;
	});
</script>

<main>
	<section>
		<div class="section-header">
			<h1 style="margin-bottom: 1rem;">Featured Themes</h1>
		</div>
		{#if !loadingFeatured}
			<div>
				<FeaturedThemes themes={featuredThemes} />
			</div>
		{:else}
			<div class="loading-featured">
				<h5>Loading</h5>
				<Stretch color="#6ea8fe" />
			</div>
		{/if}
		<div class="section-header panel-gap">
			<h1>Your Themes</h1>
			<NavigationCircle orientation="right" href="/themes/my-themes" />
		</div>
		{#if !loadingMy}
			<ThemeCarousel themes={myThemes} />
		{:else}
			<div class="loading">
				<h5>Loading</h5>
				<DoubleBounce color="#8bb9fe" />
			</div>
		{/if}
		<div class="section-header panel-gap">
			<h1>Theme Store</h1>
			<NavigationCircle orientation="right" href="/themes/theme-store" />
		</div>
		{#if !loadingAll}
			<ThemeCarousel themes={allThemes} networked={true} />
		{:else}
			<div class="loading">
				<h5>Loading</h5>
				<DoubleBounce color="#8bb9fe" />
			</div>
		{/if}
	</section>
</main>

<style lang="scss">
	@import "./+page.scss";
</style>
