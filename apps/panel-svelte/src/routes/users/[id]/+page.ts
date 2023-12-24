import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
	const { id } = params;

	const res = await fetch(`/api/users/${id}`);
	const user = await res.json();

	return { user };
};
