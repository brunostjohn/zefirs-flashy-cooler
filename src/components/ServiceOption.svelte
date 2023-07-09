<script lang="ts">
	import { goto } from "$app/navigation";
	import { sleep } from "../helpers/sleep";

	export let href: string;
	export let name: string;
	export let desc: string;

	let opt: HTMLDivElement;

	const clicked = async () => {
		goto(`/services${href}`);
		opt.classList.add("animate-click-link");
		await sleep(200);
		opt.classList.remove("animate-click-link");
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="selector-opt" on:click={() => clicked()} bind:this={opt}>
	<div class="image">
		<slot />
	</div>
	<div class="text">
		<p class="name">{name}</p>
		<p class="desc">{desc}</p>
	</div>
</div>

<style lang="scss">
	@import "../styles/mixins.scss";

	.selector-opt {
		width: 100%;

		@include flex-center;

		transition: all 150ms ease-in-out;
		&:hover {
			background-color: rgba(var(--bs-primary-rgb), 0.15);
		}

		border-radius: 15px;

		padding: 0.6rem;
		margin-bottom: 0.3rem;
	}

	.image {
		$size: 2.4rem;

		width: $size;
		height: $size;

		margin-right: 0.7rem;

		@include flex-center;

		filter: invert(1) brightness(0.6);
	}

	.text {
		.name {
			margin: 0;
		}

		.desc {
			margin: 0;

			font-size: 13px;
		}
	}

	:global(.animate-click-link) {
		animation: clicked-link-animation 200ms both linear;
	}

	@keyframes clicked-link-animation {
		0% {
			background-color: rgba(var(--bs-primary-rgb), 0.15);
		}

		50% {
			background-color: rgba(var(--bs-primary-rgb), 0.3);
		}

		100% {
			background-color: rgba(var(--bs-primary-rgb), 0);
		}
	}
</style>
