import Image from 'next/image'
import TextInput from '../Elements/Forms/TextInput'
import Submit from '../Elements/Forms/Submit'
import { useTranslations } from 'next-intl';

export default function LoginContainer({ }) {
    const t = useTranslations('Login');

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
                            <h1 className="font-bold text-2xl md:text-5xl">{t('login-header')}</h1>
                            <h2 className="font-semibold text-lg md:text-2xl">{t('login-subheader')}</h2>
                        </div>
                    </div>

                    <div className="flex justify-center text-center pt-6">
                        <div>
                            <TextInput label={t('login-username')} className="h-10"></TextInput>
                            <TextInput label={t('login-password')} className="h-10 mt-4"></TextInput>
                            <Submit value={t('login-button')} className="h-10 mt-4 w-full"></Submit>
                        </div>
                    </div>
                </div>
            </div>
        </>
    )
}