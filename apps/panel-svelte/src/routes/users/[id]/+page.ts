import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
	const { id } = params;

	const res = await fetch(`/api/users/${id}`);

	if (res.status == 404) {
		console.log(404);
		error(404, "Not found");
	}
	
	const user = await res.json();

	return { user };
};
