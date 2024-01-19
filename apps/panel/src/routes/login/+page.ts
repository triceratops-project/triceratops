import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
	const res = await fetch("/api/auth/oauth");

	const oauthProviders = await res.json();

	return { oauthProviders };
};
