<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    export let users = data.users;
</script>

<div>
    <h1>Users</h1>
    <ul>
        {#each users as user}
            <li>
                <div>
                    <p>Id: {user.id}</p>
                    <p>Name: {user.firstName ?? "Not Set"}</p>
                    <p>Email: {user.email}</p>
                </div>
            </li>
        {/each}
    </ul>
</div>
