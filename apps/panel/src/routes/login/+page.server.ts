import type {Actions, PageServerLoad} from "./$types";
import { env } from "$env/dynamic/public";
import {redirect} from "@sveltejs/kit";

export const load = (async ({ cookies }) => {
    const session = cookies.get('session');
    const user = cookies.get('user');
    console.log(session)

    if (session && user) {
        return redirect(302, "/")
    }

    return {};
}) satisfies PageServerLoad;

export const actions = {
    default: async (event) => {
        const request = event.request;
        const formData = await request.formData();
        const username = formData.get("username");
        const password = formData.get("password");

        const req = await fetch(`${env.PUBLIC_API_URL}/api/auth/login`, {
            method: "POST",
            body: JSON.stringify({ username, password }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const res = await req.json();

        if (res.message) {
            return { success: false, message: res.message };
        }

        event.cookies.set("session", JSON.stringify(res.session), {path: "/"})
        event.cookies.set("user", JSON.stringify(res.user), {path: "/"})

        return { success: true, user: res.user, session: res.session }
    },
} satisfies Actions;
