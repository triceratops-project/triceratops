import Link from 'next/link'


export default function Home({}) {
	return (
		<>
			<h1>yo</h1>
			<Link href={`/cool-page`}>
				cool page
			</Link>
		</>
	)
}