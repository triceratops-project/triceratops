<script lang="ts">
    // @ts-ignore
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faDiscord,
        faGoogle,
        faMicrosoft,
        faWhmcs,
        type IconDefinition,
    } from "@fortawesome/free-brands-svg-icons";

    import { faCircleDashed, faUserShield } from "@fortawesome/pro-solid-svg-icons";

    import type { PageData } from "./$types";

    export let data: PageData;
    export let oauthProviders = data.oauthProviders;

    const logos: { [key: string]: IconDefinition } = {
        custom: faUserShield,
        discord: faDiscord,
        google: faGoogle,
        microsoft: faMicrosoft,
        okta: faCircleDashed,
        whmcs: faWhmcs
    };

    async function oauthRedirect(provider: string) {
        const res = await fetch(`/api/auth/oauth/${provider}`);

        const resJson = await res.json();

        window.location = resJson.url;
    }
</script>

<div class="hidden xl:block">
    <img
        src="/assets/better_background.jpg"
        alt="Background"
        class="object-cover absolute w-full h-full"
    />
</div>

<div>
    <div class="flex justify-end p-6 xl:p-24 h-screen w-full">
        <div
            class="flex flex-col items-center bg-neutral-900 rounded-lg w-full md:w-4/12 xl:w-3/12 h-full xl:relative"
        >
            <div class="flex flex-col items-center pt-8">
                <div class="relative h-28 w-28">
                    <img
                        src="/assets/logos/triceratops.png"
                        alt="Triceratops"
                    />
                </div>
                <hr class="w-16 h-0.5 rounded border-0 bg-neutral-700" />
            </div>
            <div class="flex justify-center text-center pt-6">
                <div>
                    <h1 class="font-bold text-2xl md:text-5xl">Login</h1>
                    <h2 class="font-medium text-md md:text-lg mt-0.5">
                        Enter login details
                    </h2>
                </div>
            </div>

            <div
                class="flex flex-col justify-center text-center lg:px-8 px-6 pt-6"
            >
                <!-- {isUnauthorized && <p className='text-start pl-2 text-red-600'>Invalid username or password.</p>} -->
                <!-- {errorOccurred && <p className='text-start pl-2 text-red-600'>An unexpected error has occurred.</p>} -->
                <form>
                    <input
                        name="username"
                        type="text"
                        placeholder="Email or Username"
                        class="h-10 w-full appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-500 focus:border-2 border-pink-400 focus:ring ring-pink-400/40 outline-none"
                    />
                    <input
                        name="password"
                        type="password"
                        placeholder="Password"
                        class="h-10 w-full mt-2 appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-500 focus:border-2 border-pink-400 focus:ring ring-pink-400/40 outline-none"
                    />
                    <input
                        type="submit"
                        value="Login"
                        class="font-bold h-10 mt-4 w-full appearance-none rounded-md bg-pink-500 hover:bg-pink-400 transition cursor-pointer active:ring ring-pink-400/40"
                    />
                    <p class="text-start pl-2 text-neutral-500">
                        or, <a
                            href="/register"
                            class="text-pink-400 hover:text-pink-300"
                            >register for an account.</a
                        >
                    </p>
                </form>
            </div>
            <hr
                class="w-16 h-0.5 rounded border-0 bg-neutral-700 mt-4 my-2 justify-center"
            />
            <div class="flex flex-col space-y-2 w-full lg:px-8 px-6">
                {#each oauthProviders.services as service}
                    <button
                        on:click={() => {
                            oauthRedirect(service.iden).then();
                        }}
                        class="flex justify-center items-center font-semibold h-10 mt-4 w-full appearance-none rounded-md bg-[#5865F2] hover:bg-[#6f78dc] transition cursor-pointer active:ring ring-[#5865F2]/40"
                        ><p>
                            <FontAwesomeIcon
                                icon={logos[service.iden]}
                                class="mr-1"
                            />
                            {service.name}
                        </p>
                    </button>
                {/each}
            </div>
        </div>
    </div>
</div>
