<script>
    import RegisterComponent from "$components/RegisterComponent.svelte";
    export let form;
</script>

<div class="hidden xl:block">
    <img
        src="/assets/better_background.jpg"
        alt="Background"
        class="object-cover absolute w-full h-full"
    />
</div>

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
                <h1 class="font-bold text-2xl md:text-5xl">Register</h1>
                <h2 class="font-medium text-md md:text-lg mt-0.5">
                    Register your account
                </h2>
            </div>
        </div>

        <div class="flex flex-col justify-center text-center px-12 pt-6">
            {#if form && !form.success}
                <p class='text-start pl-2 text-red-600'>{form.message}</p>
            {/if}
            <!-- {errorOccurred && <p className='text-start pl-2 text-red-600'>An unexpected error has occurred.</p>} -->
            <form method="POST">
                <input
                        name="email"
                        type="text"
                        placeholder="Email"
                        class="h-10 w-full appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-500 focus:border-2 border-pink-400 focus:ring ring-pink-400/40 outline-none"
                />
                <input
                        name="username"
                        type="text"
                        placeholder="Username"
                        class="h-10 w-full mt-2 appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-500 focus:border-2 border-pink-400 focus:ring ring-pink-400/40 outline-none"
                />
                <input
                        name="password"
                        type="password"
                        placeholder="Password"
                        class="h-10 w-full mt-2 appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-500 focus:border-2 border-pink-400 focus:ring ring-pink-400/40 outline-none"
                />
                <input
                        type="submit"
                        value="Register"
                        class="font-bold h-10 mt-4 w-full appearance-none rounded-md bg-pink-500 hover:bg-pink-400 transition cursor-pointer active:ring ring-pink-400/40"
                />
                <p class="text-start pl-2 text-neutral-500">
                    or, <a
                        href="/login"
                        class="text-pink-400 hover:text-pink-300"
                >login to an account.</a
                >
                </p>
            </form>
        </div>
    </div>
</div>
