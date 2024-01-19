import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
	const res = await fetch("/api/auth/oauth");

	const beanbo = await res.json();

	return { beanbo };
};
