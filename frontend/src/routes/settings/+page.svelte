<script lang="ts">
	import zefirLogo from "$lib/assets/128x128.png";
	import zefirPic from "$lib/assets/about_card.jpg";
</script>

<h1 class="h2 inline-flex align-center items-center">
	About
	<img src={zefirLogo} alt="Zefir's Flashy Cooler" class="inline-block w-9 h-9 ml-3 mr-1" />
	Zefir's Flashy Cooler.
</h1>
<p class="mt-4 p">
	Zefir's Flashy Cooler is a simple app that allows you to control your LCD CPU cooler without the
	manufacturers' limitations and bloatware.
</p>
<br />
<div class="float-right relative w-64 h-64 ml-8 mb-12">
	<img src={zefirPic} alt="Zefir" class="w-64 h-64 absolute top-0 left-0 z-10" />
	<div class="w-64 h-64 bg-primary-700 absolute z-0 -inset-x-4 inset-y-4" aria-hidden="true" />
</div>

<p class="text-justify">
	This software comes AS IS with no warranties or guaranties of ANY kind. If you feel it's shady,
	please check its source out on GitHub. Remember, this is third party software based on a
	reverse-engineering effort. If something breaks, sorry but I'm not responsible. I do my best so
	that this app works as intended and use it myself but I can't promise anything.
</p>

<p class="mt-6">Made with <span class="emoji-text">&#10084;&#65039;</span> in Cork, Ireland.</p>

<p class="mt-6">
	The picture on the right is my cat Zefir. He's supported me emotionally through the development of
	this app.
</p>

<p class="mt-6">
	You can support me by <a
		href="https://github.com/sponsors/brunostjohn"
		target="_blank"
		class="anchor">buying me a coffee or a beer</a
	>. I'd appreciate it a lot. You can also support me by spreading the word about this app. The more
	people use it, the more I'll be motivated to keep it up to date.
</p>

<div class="mt-10 flex align-center items-center gap-4">
	<small class="inline-flex align-center items-center text-surface-300">
		App: <code class="code ml-2">{window.__APP_VERSION__ ?? "0.0.0"}</code>
	</small>
	<small class="inline-flex align-center items-center text-surface-300">
		UI: <code class="code ml-2">{window.__UI_VERSION__ ?? "0.0.0"}</code>
	</small>
	<small class="inline-flex align-center items-center text-surface-300">
		Ultralight: <code class="code ml-2">{window.__UL_VERSION__ ?? "0.0.0"}</code>
	</small>
	<small class="inline-flex align-center items-center text-surface-300">
		Backend: <code class="code ml-2">{window.__BACKEND_VERSION__ ?? "0.0.0"}</code>
	</small>
	<small class="inline-flex align-center items-center text-surface-300">
		LibreHardwareMonitor: <code class="code ml-2">{window.__LIBRE_VERSION__ ?? "0.0.0"}</code>
	</small>
</div>

<style>
	@font-face {
		font-family: "color-emoji";
		src: local("Apple Color Emoji"), local("Segoe UI Emoji"), local("Segoe UI Symbol"),
			local("Noto Color Emoji");
	}

	.emoji-text {
		font-family: "color-emoji";
	}
</style>
