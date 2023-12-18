import { error } from "@sveltejs/kit";

export const load = async ({ url: { searchParams } }) => {
	const fsName = searchParams.get("fsName");
	if (!fsName) throw error(404, "Not found");

	return {
		fsName,
	};
};
