<script lang="ts">
	import { onMount } from "svelte";
	import { sleep } from "../helpers/sleep";

	export let onclick: () => void | Promise<void> = () => {
		return;
	};
	export let href = "";

	let background: HTMLAnchorElement;

	const transitionDurationMs = 150;

	const clicked = async () => {
		background.classList.add("animate-circle-navi-aaaaaaaa");
		await sleep(transitionDurationMs);
		background.classList.remove("animate-circle-navi-aaaaaaaa");
		onclick();
	};

	export let orientation: "left" | "right" = "left";

	let canvas: HTMLCanvasElement;

	onMount(() => {
		animateCanvas();
	});

	let gradientNumber = Math.random() * (500 - 100) + 100;
	let anotherGradientNumber = Math.random() * (500 - 100) + 100;
	let flag = false;

	const animateCanvas = () => {
		if (canvas) {
			const ctx = canvas.getContext("2d");

			const gradient = ctx?.createLinearGradient(0, 0, gradientNumber, anotherGradientNumber);

			gradient?.addColorStop(0, "#bd34fe");
			gradient?.addColorStop(1, "#41d1ff");

			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-expect-error
			ctx.fillStyle = gradient;

			ctx?.fillRect(0, 0, canvas.width, canvas.height);

			if (gradientNumber > 500) {
				flag = true;
			} else if (gradientNumber <= 100) {
				flag = false;
			}

			if (flag) {
				gradientNumber--;
				anotherGradientNumber++;
			} else {
				gradientNumber++;
				anotherGradientNumber--;
			}

			window.requestAnimationFrame(animateCanvas);
		}
	};
</script>

<a
	bind:this={background}
	class={"btn-container"}
	on:click={clicked}
	{href}
	style={`transform: rotate(${orientation == "left" ? 0 : 180}deg);`}
>
	<canvas class="bg" bind:this={canvas} />
	<svg xmlns="http://www.w3.org/2000/svg" class="arrow" viewBox="0 0 448 512"
		><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
			d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.2 288 416 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-306.7 0L214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z"
		/></svg
	>
</a>

<style lang="scss">
	@import "../styles/mixins.scss";

	.bg {
		position: absolute;
		width: 100%;
		height: 100%;
	}

	$arrow-size: 2.5rem;

	.btn-container {
		width: $arrow-size;
		height: $arrow-size;

		position: relative;
		overflow: hidden;

		@include flex-center;

		border-radius: 100%;
		// background-color: var(--bs-primary);
		// background: -webkit-linear-gradient(190deg, #bd34fe 15%, #41d1ff 100%);
		filter: brightness(100%);

		margin: 1rem;

		transition: filter 150ms ease-in-out;

		&:hover {
			filter: brightness(80%);
		}

		.arrow {
			width: $arrow-size * 0.72;
			height: $arrow-size * 0.72;

			@include white-svg-filter;
		}
	}

	:global(.animate-circle-navi-aaaaaaaa) {
		animation: circle-navi-animation 150ms both linear;
	}

	@keyframes circle-navi-animation {
		0% {
			filter: brightness(80%);
		}

		50% {
			filter: brightness(60%);
		}

		100% {
			filter: brightness(100%);
		}
	}
</style>
