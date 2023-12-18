import { purgeCss } from "vite-plugin-tailwind-purgecss";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import * as fs from "fs";

export default defineConfig({
	plugins: [sveltekit(), purgeCss()],
	define: {
		__UI_VERSION__: getUIVersion(),
		__APP_VERSION__: getAppVersion(),
		__BACKEND_VERSION__: getBackendVersion(),
		__LIBRE_VERSION__: getLibreVesion(),
		__UL_VERSION__: getULVersion(),
	},
});

function getAppVersion() {
	try {
		const tauriConfig = JSON.parse(fs.readFileSync("../tauri.conf.json", "utf-8"));
		return `"${tauriConfig.package.version}"`;
	} catch {
		return `"0.0.0"`;
	}
}

function getBackendVersion() {
	try {
		const cargoToml = fs.readFileSync("../Cargo.toml", "utf-8");
		const regex = /version = "([0-9.]+)"/;
		const matches = cargoToml.match(regex);
		if (matches) {
			return `"${matches[1]}"`;
		} else {
			return `"0.0.0"`;
		}
	} catch {
		return `"0.0.0"`;
	}
}

function getLibreVesion() {
	try {
		const cargoToml = fs.readFileSync("../crates/librehardwaremonitor-rs/Cargo.toml", "utf-8");
		const regex = /version = "([0-9.]+)"/;
		const matches = cargoToml.match(regex);
		if (matches) {
			return `"${matches[1]}"`;
		} else {
			return `"0.0.0"`;
		}
	} catch {
		return `"0.0.0"`;
	}
}

function getULVersion() {
	try {
		const cargoToml = fs.readFileSync("../crates/ultralight/Cargo.toml", "utf-8");
		const regex = /version = "([0-9.]+)"/;
		const matches = cargoToml.match(regex);
		if (matches) {
			return `"${matches[1]}"`;
		} else {
			return `"0.0.0"`;
		}
	} catch {
		return `"0.0.0"`;
	}
}

function getUIVersion() {
	try {
		const packageJson = JSON.parse(fs.readFileSync("./package.json", "utf-8"));
		const matches = packageJson.version;
		if (matches) {
			return `"${matches}"`;
		} else {
			return `"0.0.0"`;
		}
	} catch {
		return `"0.0.0"`;
	}
}
