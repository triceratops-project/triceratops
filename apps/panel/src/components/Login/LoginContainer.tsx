import Image from 'next/image'
import TextInput from '../Elements/Forms/TextInput'
import Submit from '../Elements/Forms/Submit'

export default function LoginContainer({ }) {
    return (
        <>
            <div className="flex justify-end p-6 xl:p-24 h-screen w-full">
                <div className="bg-neutral-900 rounded-lg w-full xl:w-4/12 h-full xl:relative">
                    <div className="flex justify-center pt-8">
                        <div className="relative h-28 w-28">
                            <Image 
                                fill
                                src="/assets/logos/triceratops.png"
                                alt="Triceratops"
                            />
                        </div>
                    </div>
                    <div className="flex justify-center text-center pt-6">
                        <div>
                            <h1 className="font-bold text-2xl md:text-5xl">Login</h1>
                            <h2 className="font-semibold text-lg md:text-2xl">Login for logins!</h2>
                        </div>
                    </div>

                    <div className="flex justify-center text-center pt-6">
                        <div>
                            <TextInput label="Email" className="h-10"></TextInput>
                            <TextInput label="Password" className="h-10 mt-4"></TextInput>
                            <Submit value="Login" className="h-10 mt-4 w-full"></Submit>
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}