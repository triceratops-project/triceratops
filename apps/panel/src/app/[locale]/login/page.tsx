import LoginContainer from '@components/Login/LoginContainer'
import Triceratops from '@/components/Login/Triceratops'

export default function LoginPage({ }) {

	return (
		<>
			<div className="hidden xl:block">
				<Triceratops />
			</div>

			<div className="">
				<div className="">
					<LoginContainer />
				</div>
			</div>
		</>
	)
}