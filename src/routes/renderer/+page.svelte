<div class="col-container">
	<div class="col" id="larger">
		<div id="scrollContainer">
			<h1>About the theme</h1>
			<hr />
			<h2 id="themeName">Theme name</h2>
			<p id="themeDescription">Theme description</p>
			<div id="parameters">
				<hr id="separator" />
				<h2>Controllable parameters</h2>
				<form id="parameterContainer" />
				<div class="btn-group">
					<button type="button" class="btn btn-primary" id="apply">Apply</button>
					<button type="button" class="btn btn-secondary" id="reset"
						>Reset parameters to default values</button
					>
				</div>
			</div>
		</div>
	</div>
	<div class="col" id="smaller">
		<div id="rendering">
			<h4>Currently rendering:</h4>
			<div class="card">
				<!-- svelte-ignore a11y-missing-attribute -->
				<img src="" class="card-img-top" id="preview" />
				<div class="card-body">
					<h5 class="card-title" id="title">Nothing</h5>
				</div>
			</div>
		</div>
		<button id="btn" class="btn btn-primary btn-lg">Start Rendering</button>
	</div>
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
