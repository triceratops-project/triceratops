import Image from 'next/image'

export default function Logo({ }) {
    return (
        <>
            <div className="">
                <Image
                    fill
                    src="/assets/logos/triceratops.png"
                    alt="Triceratops"
                />
            </div>
        </>
    )
}