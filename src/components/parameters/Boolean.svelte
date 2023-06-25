<script lang="ts">
	import type { Parameter } from "../../helpers/themeTools";
	export let parameter: Parameter;

	const { name, display_as, default: def } = parameter;
</script>

<div class="form-check form-switch">
	<input class="form-check-input" type="checkbox" role="switch" id="flexSwitchCheckDefault" />
	<label class="form-check-label" for="flexSwitchCheckDefault">{display_as}</label>
</div>
