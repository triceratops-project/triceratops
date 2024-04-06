import type { PageLoad } from "./$types";
import { env } from '$env/dynamic/public'

export const load: PageLoad = async () => {
	const res = await fetch(`${env.PUBLIC_API_URL}/api/auth/oauth`);

	const oauthProviders = await res.json();

	return { oauthProviders };
};
