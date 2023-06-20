import { getThemes } from "../../../helpers/themeTools.js";

export const load = async (_) => {
	return {
		themes: await getThemes(),
	};
};
