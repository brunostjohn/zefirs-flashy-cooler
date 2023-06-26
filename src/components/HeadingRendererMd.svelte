<script lang="ts">
	export let depth: number;
	export let raw: string;
</script>

{#if depth === 1}
	<h3><slot /></h3>
{:else if depth === 2}
	<h4><slot /></h4>
{:else if depth === 3}
	<h5><slot /></h5>
{:else if depth === 4}
	<h6><slot /></h6>
{:else if depth === 5}
	<b><slot /></b>
{:else if depth === 6}
	<b><slot /></b>
{:else}
	{raw}
{/if}
