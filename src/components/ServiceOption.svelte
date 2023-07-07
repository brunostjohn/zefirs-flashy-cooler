<script lang="ts">
	import { page } from "$app/stores";
	import { goto } from "$app/navigation";
	import { sleep } from "../helpers/sleep";

	export let href: string;
	export let name: string;
	export let desc: string;

	let opt: HTMLDivElement;

	const clicked = async () => {
		opt.classList.add("animate-circle-navi-aaaaaaaa");
		await sleep(300);
		opt.classList.remove("animate-circle-navi-aaaaaaaa");
		goto(`/services${href}`);
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="selector-opt"
	on:click={() => clicked()}
	bind:this={opt}
	style={$page.url.pathname === `/services${href}`
		? `border: 1px solid rgba(255, 255, 255, 0.05);`
		: undefined}
>
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
			border: 1px solid transparent !important;
		}

		border-radius: 15px;

		padding: 0.6rem;
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
		animation: clicked-link-animation 150ms both linear;
		border: 1px solid transparent !important;
	}

	@keyframes clicked-link-animation {
		0% {
			background-color: rgba(var(--bs-primary-rgb), 0.15);
		}

		50% {
			background-color: rgba(var(--bs-primary-rgb), 0.3);
		}

		100% {
			background-color: rgba(var(--bs-primary-rgb), 0.15);
		}
	}
</style>
