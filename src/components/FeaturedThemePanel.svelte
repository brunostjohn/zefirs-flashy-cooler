<script lang="ts">
	import type { Theme } from "../helpers/themeTools";
	import SvelteMarkdown from "svelte-markdown";
	import HeadingRendererMd from "./HeadingRendererMd.svelte";

	export let theme: Theme;
</script>

<div class="panel-container">
	<div class="featured-theme">
		<img src={theme.image_src} alt="Featured theme" class="bg-img" />
		<div class="short-info" style={`color: ${theme.colour};`}>
			<h3>
				{theme.name}
			</h3>
			<h4>by {theme.author}</h4>
			<img src={theme.image_src} alt="Preview" class="preview-img" />
		</div>
		<div class="text-container" style={`color: ${theme.colour};`}>
			<div class="theme-desc">
				<SvelteMarkdown source={theme.description} renderers={{ heading: HeadingRendererMd }} />
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	@import "../styles/mixins.scss";
	.panel-container {
		width: 55rem;
		height: 25rem;
		position: relative;

		@include flex-center;

		&:hover {
			.featured-theme {
				width: 54.6rem;
				height: 24.6em;
			}
		}
	}

	.featured-theme {
		width: 55rem;
		height: 25rem;
		position: absolute;
		border-radius: 15px;
		overflow: hidden;

		transition: all 100ms ease-in-out;

		.bg-img {
			width: 120%;
			height: 120%;

			position: relative;
			left: -10%;
			top: -10%;

			object-fit: cover;
			z-index: -5;
			filter: blur(20px) brightness(90%);
		}

		.short-info {
			z-index: 200;
			top: 0;
			left: 3%;
			height: 100%;
			position: absolute;

			@include flex-center;
			flex-direction: column;

			h3 {
				max-width: 20rem;
				overflow: hidden;
				max-height: 2.4rem;
				text-overflow: ellipsis;
				white-space: nowrap;
			}

			h4 {
			}

			.preview-img {
				$img-size: 15rem;

				width: $img-size;
				height: $img-size;

				margin-top: 1rem;

				border-radius: 15px;
			}
		}

		.text-container {
			position: absolute;
			z-index: 200;
			bottom: 0;
			right: 3%;

			height: 100%;

			@include flex-center;

			width: 30rem;

			color: red;
		}

		.theme-desc {
			overflow-wrap: break-word;
			overflow: hidden;
			text-align: justify;
			text-overflow: ellipsis;
			display: -webkit-box;
			-webkit-line-clamp: 9;
			line-clamp: 9;
			-webkit-box-orient: vertical;
		}
	}
</style>
