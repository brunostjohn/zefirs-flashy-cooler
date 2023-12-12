<script lang="ts">
	import { Icon, type IconSource } from "@steeze-ui/svelte-icon";

	export let icon: IconSource;
	export let color:
		| "primary"
		| "secondary"
		| "surface"
		| "error"
		| "warning"
		| "success"
		| "info"
		| "tertiary" = "primary";

	const borderColors = {
		primary: "border-primary-600",
		secondary: "border-secondary-600",
		surface: "border-surface-600",
		error: "border-error-600",
		warning: "border-warning-600",
		success: "border-success-600",
		info: "border-info-600",
		tertiary: "border-tertiary-600",
	};

	const bgColors = {
		primary: "hover:bg-primary-600",
		secondary: "hover:bg-secondary-600",
		surface: "hover:bg-surface-600",
		error: "hover:bg-error-600",
		warning: "hover:bg-warning-600",
		success: "hover:bg-success-600",
		info: "hover:bg-info-600",
		tertiary: "hover:bg-tertiary-600",
	};
</script>

<button
	type="button"
	class={`btn-icon w-7 h-7 btn-icon-sm rounded-none bg-white border-${color}-600 border-[3px] transition-all hover:border-0 hover:bg-${color}-600 hover:text-white hover:rounded-[100%] ` +
		borderColors[color] +
		" " +
		bgColors[color]}
	on:click
>
	<Icon src={icon} class="w-5 h-5 text-surface-900 hover:text-surface-500 transition-all" />
</button>
