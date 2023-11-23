import Image from 'next/image'
import TextInput from '@elements/Forms/TextInput'
import Submit from '@elements/Forms/Submit'
import { useTranslations } from 'next-intl';
import PasswordInput from '@elements/Forms/PasswordInput';

export default function LoginContainer({ }) {
    const t = useTranslations('Login');

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
                            <h1 className="font-bold text-2xl md:text-5xl">{t('login-header')}</h1>
                            <h2 className="font-medium text-md md:text-lg">{t('login-subheader')}</h2>
                        </div>
                    </div>

                    <div className="flex flex-col justify-center text-center px-12 pt-6">
                        <TextInput label={t('login-username')} placeholder={t('login-username')} className="h-10 w-full"></TextInput>
                        <PasswordInput label={t('login-password')} placeholder={t('login-password')} className="h-10 w-full mt-2"></PasswordInput>
                        <Submit value={t('login-button') + " 🥸"} className="font-bold h-10 mt-4 w-full"></Submit>
                        <p className='text-start pl-2 text-neutral-600'>or, <a href='/register' className='text-pink-400 hover:text-pink-300'>register an account.</a></p>
                    </div>
                    <p className='hidden'>kill youself, now!!!?</p>
                </div>
            </div>
        </>
    )
}