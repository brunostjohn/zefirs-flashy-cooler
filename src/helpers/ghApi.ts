/* eslint-disable */
//@ts-ignore
import ColorThief from "../../node_modules/colorthief/dist/color-thief.mjs";
import type { Theme } from "../helpers/themeTools";

export const getAllThemes = async (truncateAfter: number = 0) => {
	const allThemes = await getDirContents("Themes");

	let themes = [];
	let counter = 0;

	for (const theme of allThemes) {
		const result = await fetchAndParseTheme(theme.name);
		themes.push(result);
		if (truncateAfter > 0) {
			counter++;

			if (counter >= truncateAfter) {
				break;
			}
		}
	}

	return themes;
};

export const getFeaturedThemes = async () => {
	const manifest = await getMasterManifest();

	let featuredThemes = [];

	for (const featuredTheme of manifest.featured) {
		const theme = await fetchAndParseTheme(featuredTheme);
		featuredThemes.push(theme);
	}

	return featuredThemes;
};

export const getMasterManifest = async () => {
	const restEndpoint = `https://api.github.com/repos/brunostjohn/zefirs-flashy-cooler-themes/contents/manifest.json`;

	const response = await fetch(restEndpoint, {
		headers: {
			Accept: "application/vnd.github+json",
			"X-GitHub-Api-Version": "2022-11-28",
		},
	});

	const parsedObject = await response.json();

	const manifestFile = await fetch(parsedObject.download_url);
	const manifest = await manifestFile.json();

	return manifest;
};

export const getDirContents = async (dirPath: string = "") => {
	const restEndpoint = `https://api.github.com/repos/brunostjohn/zefirs-flashy-cooler-themes/contents/${encodeURIComponent(
		dirPath
	)}`;

	const response = await fetch(restEndpoint, {
		headers: {
			Accept: "application/vnd.github+json",
			"X-GitHub-Api-Version": "2022-11-28",
		},
	});

	const parsedObject = await response.json();

	return parsedObject;
};

export const fetchAndParseTheme = async (themePath: string): Promise<Theme> => {
	const restEndpoint = `https://api.github.com/repos/brunostjohn/zefirs-flashy-cooler-themes/contents/Themes/${encodeURIComponent(
		themePath
	)}`;

	const response = await fetch(restEndpoint, {
		headers: {
			Accept: "application/vnd.github+json",
			"X-GitHub-Api-Version": "2022-11-28",
		},
	});

	const parsedObject = await response.json();

	const manifestUrl = parsedObject.find(
		(file: { name: string }) => file.name === "theme.json"
	).download_url;

	const ct = new ColorThief();

	const manifestFile = await fetch(manifestUrl);
	const manifest = await manifestFile.json();

	const image_src = parsedObject.find(
		(file: { name: string }) => file.name === "preview.jpg"
	).download_url;

	const tempImage = new Image();
	tempImage.src = image_src;

	// const colours = ct.getColor(tempImage) as number[];
	const colour = "#000000";

	const themeObject: Theme = {
		description: manifest.description,
		author: manifest.author,
		name: manifest.name,
		fs_name: themePath,
		image_src,
		colour,
		// colour: `#${colours[0].toString(16)}${colours[1].toString(16)}${colours[2].toString(16)}`,
	};

	return themeObject;
};
