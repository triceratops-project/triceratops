import Link from 'next/link'

export default function Home({ }) {
	return (
		<>
			<h1>boobs</h1>
			<Link href={`/cool-page`}>
				cool page
			</Link>
		</>
	)
}