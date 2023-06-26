<script lang="ts">
	export let placeholder = "Search";
	export let input: string;
</script>

<input
	type="text"
	class="form-control searchbar"
	placeholder={"\u2315  " + placeholder}
	bind:value={input}
/>

<style lang="scss">
	.searchbar {
		// max-width: 25rem;
		height: 3rem;

		background-color: #00000000;

		&::placeholder {
			font-weight: 100;
			font-size: 1.2rem;

			opacity: 0.2;
		}
	}
</style>
