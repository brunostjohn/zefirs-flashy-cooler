<script lang="ts">
	export let content: string;
</script>

<div class="chip">
	<div class="image">
		<slot />
	</div>
	<p>{content}</p>
</div>

<style lang="scss">
	@import "../styles/mixins.scss";

	.chip {
		@include flex-center;

		flex-shrink: 0;

		margin-right: 0.6rem;

		border: 1px solid var(--bs-body-color);
		border-radius: 20px;

		min-width: 10%;
		padding: 0.5rem;
		padding-left: 0;
		padding-right: 0.8rem;

		opacity: 70%;
		filter: brightness(0.7);

		.image {
			width: 1rem;
			height: 1rem;
			margin-right: 0.5rem;
			margin-left: 0.5rem;

			@include flex-center;

			filter: invert(84%) sepia(6%) saturate(346%) hue-rotate(169deg) brightness(85%) contrast(92%);
		}

		p {
			margin: 0;
		}
	}
</style>
