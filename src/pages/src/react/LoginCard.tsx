import { Github, Twitter } from './buttons';
import { Transition } from '@headlessui/react';
import { useState, Fragment, ChangeEvent } from 'react';
import { XCircleIcon } from '@heroicons/react/24/solid';

const LoginCard = (props: { app }) => {
	const [loading, setLoading] = useState(false);
	const [loginFailed, setLoginFailed] = useState({ state: false, msg: '' });
	const [loginForm, setLoginForm] = useState({ email: '', password: '', remember: false });

	const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
		const { name, value } = event.target;
		setLoginForm((form) => ({ ...form, [name]: value }));
	};

	const handleSubmit = (event: any) => {
		event.preventDefault();
		setLoading(true);

		fetch(`/${props.app.prefix}/api/login`, {
			method: 'POST',
			body: JSON.stringify(loginForm),
			headers: { 'Content-Type': 'application/json' }
		})
			.then(async (response) => {
				if (response.status === 200) {
					window.location.href = '/';
				} else {
					const body = await response.json();
					setLoading(false);
					setLoginFailed({ state: true, msg: body.message });
					setTimeout(() => setLoginFailed((data) => ({ ...data, state: false })), 3500);
				}
			})
			.catch(() => {
				setLoading(false);
				setLoginFailed({ state: true, msg: 'Unknown error occured.' });
				setTimeout(() => setLoginFailed((data) => ({ ...data, state: false })), 3500);
			});
	};

	return (
		<Fragment>
			<div className="mt-5">
				<form className="space-y-6" onSubmit={handleSubmit}>
					<div>
						<label for="email" className="block text-sm font-medium leading-6 text-zinc-900">
							Email address
						</label>
						<div className="mt-1">
							<input
								required
								id="email"
								name="email"
								type="email"
								disabled={loading}
								value={loginForm.email}
								onChange={handleChange}
								placeholder="james@bond.com"
								autoComplete="email"
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-${props.app.accent}-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
						</div>
					</div>
					<div>
						<label for="password" className="block text-sm font-medium leading-6 text-zinc-900">
							Password
						</label>
						<div className="mt-1">
							<input
								required
								id="password"
								name="password"
								type="password"
								disabled={loading}
								value={loginForm.password}
								onChange={handleChange}
								placeholder="••••••••"
								autoComplete="current-password"
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-${props.app.accent}-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
						</div>
					</div>

					<div className="flex items-center justify-between">
						<div className="flex items-center">
							<input
								id="remember"
								name="remember"
								type="checkbox"
								disabled={loading}
								checked={loginForm.remember}
								onChange={(event) => setLoginForm((form) => ({ ...form, remember: !form.remember }))}
								className={`transition h-4 w-4 rounded border-zinc-300 text-${props.app.accent}-600 focus:ring-${props.app.accent}-600 disabled:opacity-70`}
							/>
							<label for="remember" className="ml-3 block text-sm leading-6 text-zinc-900">
								Remember me
							</label>
						</div>
					</div>

					<div>
						<button
							type="submit"
							disabled={loading}
							className={`transition flex w-full justify-center rounded-md bg-${props.app.accent}-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-${props.app.accent}-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-${props.app.accent}-600 disabled:cursor-default disabled:bg-${props.app.accent}-500 disabled:opacity-80 disabled:hover:opacity-80`}>
							{loading ? 'Logging in...' : 'Sign in'}
						</button>
					</div>
				</form>

				<div>
					<div className="relative mt-10">
						<div className="absolute inset-0 flex items-center" aria-hidden="true">
							<div className="w-full border-t border-zinc-200"></div>
						</div>
						<div className="relative flex justify-center text-sm font-medium leading-6">
							<span className="bg-white px-6 text-zinc-900">Or continue with</span>
						</div>
					</div>

					<div className="mt-6 grid grid-cols-2 gap-4">
						<Twitter />
						<Github />
					</div>
				</div>
			</div>
			<div aria-live="assertive" className="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6">
				<div className="flex w-full flex-col items-center space-y-4 sm:items-end">
					<Transition
						as={Fragment}
						show={loginFailed.state}
						enter="transform ease-out duration-300 transition"
						enterFrom="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
						enterTo="translate-y-0 opacity-100 sm:translate-x-0"
						leave="transition ease-in duration-200"
						leaveFrom="opacity-100"
						leaveTo="opacity-0">
						<div className="pointer-events-auto w-full max-w-[25rem] overflow-hidden rounded-md bg-red-50 shadow ring-1 ring-red-100">
							<div className="p-3">
								<div className="flex items-start">
									<div className="flex-shrink-0">
										<XCircleIcon className="h-6 w-6 text-red-600" aria-hidden="true" />
									</div>
									<div className="ml-3 w-0 flex-1 pt-0.5">
										<p className="text-sm font-medium text-red-600">Failed to login!</p>
										<p className="mt-1 text-sm text-red-500">{loginFailed.msg}</p>
									</div>
								</div>
							</div>
						</div>
					</Transition>
				</div>
			</div>
		</Fragment>
	);
};

export default LoginCard;
