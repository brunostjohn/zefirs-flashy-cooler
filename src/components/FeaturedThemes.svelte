<script lang="ts">
	/* eslint-disable */
	//@ts-ignore
	import Carousel from "svelte-carousel";
	import type { Theme } from "../helpers/themeTools";
	import FeaturedThemePanel from "./FeaturedThemePanel.svelte";

	export let themes: Theme[];
</script>

{#if themes.length === 0}
	<div class="featured-failed">
		<h3>Failed to fetch themes.</h3>
		<p>Check your internet connection.</p>
	</div>
{:else}
	<Carousel autoplay autoplayDuration={5000} pauseOnFocus>
		{#each themes as theme}
			<FeaturedThemePanel {theme} />
		{/each}
	</Carousel>
{/if}

<style lang="scss">
	@import "../styles/mixins.scss";

	.featured-failed {
		@include flex-center;
		flex-direction: column;
		height: 25rem;

		h3,
		p {
			opacity: 40%;
		}

		h3 {
			font-weight: 300;
		}

		p {
			font-weight: 200;
		}
	}
</style>
