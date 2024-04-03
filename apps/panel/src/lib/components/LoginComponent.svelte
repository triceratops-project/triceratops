<script>
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faDiscord, faDribbble } from "@fortawesome/free-brands-svg-icons";
    import { faBulldozer } from "@fortawesome/pro-duotone-svg-icons";

    async function beanBoozled() {
        let beans = await (await fetch("/api/auth/oath")).json;

        return await beans;
    }
    // async function handleDiscordOauth() {
    //     const res = await fetch("/api/auth/oauth/discord");
    //     const resJson = await res.json();

    //     // redirect(302, resJson.url)

    //     console.info(resJson);
    //     window.location = resJson.url;
    // }
</script>


