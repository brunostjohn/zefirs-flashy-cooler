/* eslint-disable */
// @ts-ignore
import ColorThief from "../../node_modules/colorthief/dist/color-thief.mjs";
import type { Theme } from "../helpers/themeTools";

export const getAllThemes = async (page: number = 0) => {
	const restEndpoint = `https://zfcapi.brunostjohn.com/themes/${page}`;
	const response = await fetch(restEndpoint);
	const parsedObject = await response.json();

	let themes = [];

	for (const themeObj of parsedObject) {
		const image_src = themeObj.image_src;

		const manifestFile = await fetch(themeObj.manifestDl);
		const manifest = await manifestFile.json();

		const colour = "#000000";

		const theme: Theme = {
			description: manifest.description,
			author: manifest.author,
			name: manifest.name,
			fs_name: themeObj.fs_name,
			image_src,
			colour,
			// colour: `#${colours[0].toString(16)}${colours[1].toString(16)}${colours[2].toString(16)}`,
		};
		themes.push(theme);
	}

	return themes;
};

export const getFeaturedThemes = async () => {
	const manifest = await getMasterManifest();

	let featuredThemes = [];

	for (const featuredTheme of manifest) {
		const image_src = featuredTheme.image_src;

		const manifestFile = await fetch(featuredTheme.manifestDl);
		const manifest = await manifestFile.json();

		const colour = "#000000";

		const theme: Theme = {
			description: manifest.description,
			author: manifest.author,
			name: manifest.name,
			fs_name: featuredTheme.fs_name,
			image_src,
			colour,
			// colour: `#${colours[0].toString(16)}${colours[1].toString(16)}${colours[2].toString(16)}`,
		};
		featuredThemes.push(theme);
	}

	return featuredThemes;
};

export const getMasterManifest = async () => {
	const restEndpoint = `https://zfcapi.brunostjohn.com/featured`;

	const response = await fetch(restEndpoint);

	const parsedObject = await response.json();

	return parsedObject;
};

export const fetchAndParseTheme = async (themePath: string): Promise<Theme> => {
	const restEndpoint = `https://zfcapi.brunostjohn.com/theme/${encodeURIComponent(themePath)}`;

	const response = await fetch(restEndpoint);

	const parsedObject = await response.json();

	const manifestUrl = parsedObject.manifestDl;

	const ct = new ColorThief();

	const manifestFile = await fetch(manifestUrl);
	const manifest = await manifestFile.json();

	const image_src = parsedObject.image_src;

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
		dls: parsedObject.dlNum,
		// colour: `#${colours[0].toString(16)}${colours[1].toString(16)}${colours[2].toString(16)}`,
	};

	return themeObject;
};
