'use client'
import Image from 'next/image'
import TextInput from '@elements/Forms/TextInput'
import Submit from '@elements/Forms/Submit'
import { z } from "zod";
import PasswordInput from '@elements/Forms/PasswordInput';
import axios from 'axios';
import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function LoginContainer({ }) {
    const { push } = useRouter();

    const [firstName, setFirstName] = useState('');
    const [lastName, setLastName] = useState('');
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const [isFirstNameValid, setIsFirstNameValid] = useState(true);
    const [isLastNameValid, setIsLastNameValid] = useState(true);
    const [isUsernameValid, setIsUsernameValid] = useState(true);
    const [isEmailValid, setIsEmailValid] = useState(true);
    const [isPasswordValid, setIsPasswordValid] = useState(true);

    const [isUnauthorized, setIsUnauthorized] = useState(false);
    const [errorOccurred, setErrorOccurred] = useState(false);

    const nameSchema = z.string().min(1);
    const usernameSchema = z.string().min(3).max(128).refine(value => /^[A-Za-z0-9._]+$/.test(value));
    const emailSchema = z.string().email();
    const passwordSchema = z.string().min(8).max(128);

    const handleFirstNameChange = (event: any) => {
        if (nameSchema.safeParse(event.target.value).success) {
            setIsFirstNameValid(true);
        }
        else {
            setIsFirstNameValid(false);
        }

        setFirstName(event.target.value);
    };

    const handleLastNameChange = (event: any) => {
        if (nameSchema.safeParse(event.target.value).success) {
            setIsLastNameValid(true);
        }
        else {
            setIsLastNameValid(false);
        }

        setLastName(event.target.value);
    };

    const handleUsernameChange = (event: any) => {
        if (usernameSchema.safeParse(event.target.value).success) {
            setIsUsernameValid(true);
        }
        else {
            setIsUsernameValid(false);
        }

        setUsername(event.target.value);
    };

    const handleEmailChange = (event: any) => {
        if (emailSchema.safeParse(event.target.value).success) {
            setIsEmailValid(true);
        }
        else {
            setIsEmailValid(false);
        }

        setEmail(event.target.value);
    };

    const handlePasswordChange = (event: any) => {
        if (passwordSchema.safeParse(event.target.value).success) {
            setIsPasswordValid(true);
        }
        else {
            setIsPasswordValid(false);
        }

        setPassword(event.target.value);
    };


    const handleSubmit = async (event: any) => {
        event.preventDefault();

        let canContinue = true;

        if (!canContinue) {
            return;
        }

        axios.post(process.env.NEXT_PUBLIC_API_URL + '/auth/register', {
            firstName: firstName,
            lastName: lastName,
            username: username,
            email: email,
            password: password
        }).then((response) => {
            const data = response.data;

            const session = data.session
            const auth = `${session.id}.${session.token}`

            const user = data.user
            console.log(user)

            localStorage.setItem('authorization', auth);
            localStorage.setItem('user', JSON.stringify({
                id: user.id,
                username: user.username,
                email: user.email,
                firstName: user.firstName,
                lastName: user.lastName
            }));

            push('/dashboard');
        }).catch((error) => {
            if (!error?.response) {
                console.log(error);
                setErrorOccurred(true);
                return;
            }

            if (error?.response?.status == 401) {
                setIsUnauthorized(true);
                return;
            }

            setIsUnauthorized(false);
        });
    };

    return (
        <>
            <div className="flex justify-end p-6 xl:p-24 h-screen w-full">
                <div className="bg-neutral-900 rounded-lg w-full md:w-4/12 xl:w-3/12 h-full xl:relative">
                    <div className="flex flex-col items-center pt-8">
                        <div className="relative h-28 w-28">
                            <Image
                                fill
                                src="/assets/logos/triceratops.png"
                                alt="Triceratops"
                            />
                        </div>
                        <hr className='w-16 h-0.5 rounded border-0 bg-neutral-700' />
                    </div>
                    <div className="flex justify-center text-center pt-6">
                        <div>
                            <h1 className="font-bold text-2xl md:text-5xl">Register</h1>
                            <h2 className="font-medium text-md md:text-lg">Create a new account</h2>
                        </div>
                    </div>

                    <div className="flex flex-col justify-center text-center px-12 pt-6">
                        {isUnauthorized && <p className='text-start pl-2 text-red-600'>Invalid username or password.</p>}
                        {errorOccurred && <p className='text-start pl-2 text-red-600'>An unexpected error has occurred.</p>}
                        <form onSubmit={handleSubmit}>
                            <TextInput name="firstName" placeholder={"First Name"} className="h-10 w-full mt-2" onChange={handleFirstNameChange} valid={isFirstNameValid}></TextInput>
                            <TextInput name="lastName" placeholder={"Last Name"} className="h-10 w-full mt-2" onChange={handleLastNameChange} valid={isLastNameValid}></TextInput>
                            <TextInput name="username" placeholder={"Username"} className="h-10 w-full mt-2" onChange={handleUsernameChange} valid={isUsernameValid}></TextInput>
                            <TextInput name="email" placeholder={"Email"} className="h-10 w-full mt-2" onChange={handleEmailChange} valid={isEmailValid}></TextInput>
                            <PasswordInput name="password" placeholder={"Password"} className="h-10 w-full mt-2" onChange={handlePasswordChange} valid={isPasswordValid}></PasswordInput>
                            <Submit value={"Submit"} className="font-bold h-10 mt-4 w-full"></Submit>
                            <p className='text-start pl-2 text-neutral-600'>or, <a href='/register' className='text-pink-400 hover:text-pink-300'>login to an account.</a></p>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}