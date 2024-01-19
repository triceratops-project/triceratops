<script>
    // async function handleDiscordOauth() {
    //     const res = await fetch("/api/auth/oauth/discord");
    //     const resJson = await res.json();

    //     // redirect(302, resJson.url)

    //     console.info(resJson);
    //     window.location = resJson.url;
    // }
</script>

<div class="flex justify-end p-6 xl:p-24 h-screen w-full">
    <div
        class="bg-neutral-900 rounded-lg w-full md:w-4/12 xl:w-3/12 h-full xl:relative"
    >
        <div class="flex flex-col items-center pt-8">
            <div class="relative h-28 w-28">
                <img src="/assets/logos/triceratops.png" alt="Triceratops" />
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

        <div class="flex flex-col justify-center text-center px-12 pt-6">
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
        <!-- <div>
            <p>
                Sign in with <button on:click={handleDiscordOauth}
                    >Discord</button
                >
            </p>
        </div> -->
    </div>
</div>
