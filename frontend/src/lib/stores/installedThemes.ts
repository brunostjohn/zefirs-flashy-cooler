import { writable } from "svelte/store";

export type LCDDevice = "capellix" | "ttultra";

export interface Theme {
	description?: string;
	author?: string;
	name: string;
	fs_name: string;
	image_src: string;
	dls?: number;
	customisable_parameters: Parameter[];
	tested_on: LCDDevice[];
	version?: string;
	fileSizeKB?: number;
}

export interface Parameter {
	name: string;
	type: string;
	max: string;
	min: string;
	step: string;
	display_as: string;
	default: string;
}

export const themeStore = writable<Theme[]>([]);
