import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { env } from '$env/dynamic/public'

export const load: PageLoad = async ({ params }) => {
	const { provider } = params;

	const urlParams = new URLSearchParams(window.location.search);

	const res = await fetch(`${env.PUBLIC_API_URL}/api/auth/oauth/${provider}/callback`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			code: urlParams.get("code"),
			state: urlParams.get("state"),
		}),
	});

	if (res.status === 404) {
		error(404, "Not found");
	}

	const body = await res.text();

	return { body };
};
