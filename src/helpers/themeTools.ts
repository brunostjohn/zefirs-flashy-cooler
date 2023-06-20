import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { join, documentDir } from "@tauri-apps/api/path";

export interface Theme {
	description?: string;
	author?: string;
	name: string;
	fs_name: string;
	image_src: string;
	colour: string;
}

interface PreTheme {
	name: string;
	fs_name: string;
	colour: string;
}

export const getThemes = async (which: "most_used" | "all" = "all") => {
	const documents = await documentDir();
	const appDir = await join(documents, "Zefir's Flashy Cooler");
	const pre_themes: PreTheme[] =
		which === "all"
			? await invoke("get_all_themes")
			: ((await invoke("get_all_themes")) as PreTheme[]).slice(0, 10);

	let themes = pre_themes.map((theme) => {
		const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
		(theme as Theme)["image_src"] = convertFileSrc(preview_img);
		return theme;
	}) as Theme[];

	themes = themes.sort((a, b) => {
		if (a.name < b.name) {
			return -1;
		}
		if (a.name > b.name) {
			return 1;
		}
		return 0;
	});

	return themes;
};
