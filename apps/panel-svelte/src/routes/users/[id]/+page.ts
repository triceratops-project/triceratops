import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const { id } = params;

    const res = await fetch(`http://localhost:8000/api/users/${id}`);
    const user = await res.json();

    return { props: { user } };
};