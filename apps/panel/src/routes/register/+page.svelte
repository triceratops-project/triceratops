<script>
    export let form;
</script>

<div class="hidden xl:block">
    <img
            src="/assets/better_background.jpg"
            alt="Background"
            class="object-cover absolute w-full h-full"
    />
</div>

<div class="h-full w-full xl:absolute xl:top-1/2 xl:left-1/2 xl:transform xl:-translate-x-1/2 xl:-translate-y-1/2 xl:w-1/2 xl:h-1/2">
    <div class="bg-neutral-900 rounded-lg relative p-12">
        <div class="xl:grid xl:grid-cols-2 xl:gap-x-24">
            <div class="">
                <div class="flex justify-center items-center w-full h-full">
                    <img
                            src="/assets/logos/triceratops.png"
                            alt="Triceratops"
                    />
                </div>
            </div>

            <div>
                <div class="flex justify-center text-center pt-6">
                    <div>
                        <h1 class="font-bold text-2xl md:text-5xl">Register</h1>
                        <h2 class="font-medium text-md md:text-lg mt-0.5">
                            Register your account
                        </h2>
                    </div>
                </div>


                <div
                        class="flex flex-col justify-center text-center lg:px-8 px-6 pt-6 col-span-2"
                >
                    {#if form && !form.success}
                        <p class='text-start pl-2 text-red-600'>{form.message}</p>
                    {/if}
                    <!--{errorOccurred && <p className='text-start pl-2 text-red-600'>An unexpected error has occurred.</p>}-->
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
                        >login to an account</a
                        >
                        </p>
                    </form>
                </div>
            </div>
        </div>
    </div>
</div>
