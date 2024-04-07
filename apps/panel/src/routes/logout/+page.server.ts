import {redirect} from "@sveltejs/kit";
import type {PageServerLoad} from "../../../.svelte-kit/types/src/routes/login/$types";

export const load = (async ({ cookies }) => {
	cookies.delete("session", {path: "/"});
	cookies.delete("user", {path: "/"});

	return redirect(302, "/")
}) satisfies PageServerLoad;