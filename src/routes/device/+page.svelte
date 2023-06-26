<div id="literallyEverything">
	<div id="coolerImg">
		<!-- svelte-ignore a11y-missing-attribute -->
		<img id="preview" src="" />
	</div>
	<h2 id="title">Your LCD</h2>
	<p id="title" />
	<p id="vid">Vendor ID:</p>
	<p id="pid">Product ID:</p>
	<form id="parameterContainer">
		<div id="rotate">
			<hr />
			<h3>Rotate</h3>
			<div class="btn-group" role="group">
				<button type="button" class="btn btn-outline-secondary">&#8592;Left</button>
				<button type="button" class="btn btn-outline-primary">Right&#8594;</button>
			</div>
		</div>
		<div id="brightness" style="visibility: hidden;">
			<h3>Brightness</h3>
			<div class="btn-group" role="group">
				<button type="button" class="btn btn-outline-secondary">Darker (-)</button>
				<button type="button" class="btn btn-outline-primary">Brighter (+)</button>
			</div>
			<hr />
		</div>
		<div id="fpsControls" style="visibility: hidden;">
			<h3>FPS</h3>
			<input type="range" class="form-range" min="1" max="30" step="1" value="25" id="fpsInput" />
			<label for="fpsInput" class="form-label" id="fpsLabel"> Current value: </label>
			<button type="button" class="btn btn-outline-primary" id="applyFps">Apply</button>
			<hr />
		</div>
		<div id="hardwareImage" style="visibility: hidden;">
			<h3>Set hardware image</h3>
			<div class="btn-group">
				<button type="button" class="btn btn-outline-primary" id="apply">Apply</button>
			</div>
		</div>
	</form>
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
