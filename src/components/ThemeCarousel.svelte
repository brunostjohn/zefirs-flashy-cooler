<script lang="ts">
	/* eslint-disable */
	import type { Theme } from "../helpers/themeTools";
	//@ts-ignore
	import Carousel from "svelte-carousel";
	import ThemeCard from "./ThemeCard.svelte";

	export let networked = false;
	export let themes: Theme[];

	let leftArrow: HTMLDivElement;
	let rightArrow: HTMLDivElement;

	const leftArrOnClick = (cb: () => void) => {
		leftArrow.classList.remove("carousel-animation");
		leftArrow.classList.add("carousel-animation");

		cb();
		setTimeout(() => {
			leftArrow.classList.remove("carousel-animation");
		}, 150);
	};

	const rightArrOnClick = (cb: () => void) => {
		rightArrow.classList.remove("carousel-animation");
		rightArrow.classList.add("carousel-animation");

		cb();
		setTimeout(() => {
			rightArrow.classList.remove("carousel-animation");
		}, 150);
	};
</script>

{#if themes.length > 0}
	<div class="carousel-container">
		<Carousel
			particlesToShow={5}
			particlesToScroll={3}
			let:showPrevPage
			let:showNextPage
			--position="relative"
		>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div slot="prev" class="arrow-container arr-cnt-l">
				{#if themes.length > 5}
					<div
						class="arr-icon-container"
						on:click={() => {
							leftArrOnClick(showPrevPage);
						}}
						bind:this={leftArrow}
					>
						<svg
							class="arrow arl"
							xmlns="http://www.w3.org/2000/svg"
							height="1em"
							viewBox="0 0 320 512"
							><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
								d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l192 192c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256 246.6 86.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-192 192z"
							/></svg
						>
					</div>
				{/if}
			</div>
			{#each themes as theme}
				<ThemeCard
					src={theme.image_src}
					themeName={theme.name}
					textColour={theme.colour}
					fsName={theme.fs_name}
				/>
			{/each}
			{#if themes.length <= 5}
				{#each { length: 6 - themes.length } as _, i}
					<div class="carousel-placeholder" />
				{/each}
			{/if}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div slot="next" class="arrow-container arr-cnt-r">
				{#if themes.length > 5}
					<div
						class="arr-icon-container"
						on:click={() => rightArrOnClick(showNextPage)}
						bind:this={rightArrow}
					>
						<svg
							class="arrow arr"
							xmlns="http://www.w3.org/2000/svg"
							height="1em"
							viewBox="0 0 320 512"
							><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
								d="M310.6 233.4c12.5 12.5 12.5 32.8 0 45.3l-192 192c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L242.7 256 73.4 86.6c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l192 192z"
							/></svg
						>
					</div>
				{/if}
			</div>
		</Carousel>
	</div>
{:else}
	<div class="nothing-found">
		{#if networked}
			<h3>Failed to fetch themes.</h3>
			<p>Check your internet connection.</p>
		{:else}
			<h3>No themes found.</h3>
			<p>Try downloading some from the Theme Store.</p>
		{/if}
	</div>
{/if}

<style lang="scss">
	@import "../styles/mixins.scss";

	:global(.carousel-animation) {
		animation: clicked-carousel-animation 150ms both linear;
	}

	@keyframes clicked-carousel-animation {
		0% {
			background-color: rgba(0, 0, 0, 0.36);
		}
		50% {
			background-color: rgba(0, 0, 0, 0.56);
		}
		100% {
			background-color: rgba(0, 0, 0, 0.26);
		}
	}

	.carousel-placeholder {
		min-width: 12rem;
	}

	.carousel-container {
		position: relative;
	}

	.arrow-container {
		@include flex-center;
		position: absolute;
		width: 3rem;
		height: 100%;
		z-index: 1000;
		visibility: hidden;
		// pointer-events: none;

		.arrow {
			z-index: 1001;
			visibility: visible;
			// position: absolute;

			@include white-svg-filter;

			$arrow-size: 2rem;

			width: 1.5rem;
			height: 1.5rem;
		}

		.arr-icon-container {
			visibility: visible;
			background-color: rgba(0, 0, 0, 0.26);
			backdrop-filter: blur(10px);

			@include flex-center;

			border-radius: 100%;

			width: 2.2rem;
			height: 2.2rem;

			transition: background-color 100ms ease-in-out;

			&:hover {
				background-color: rgba(0, 0, 0, 0.36);
			}
		}
	}

	.arr-cnt-l {
		left: 0px;
	}

	.arr-cnt-r {
		right: 0px;
	}

	.nothing-found {
		width: 100%;
		height: 12rem;

		@include flex-center;
		flex-direction: column;

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
