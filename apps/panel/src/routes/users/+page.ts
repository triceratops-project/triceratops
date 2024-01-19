import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
	const res = await fetch("/api/users");

	const users = await res.json();

	return { users };
};
