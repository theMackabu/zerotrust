import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { validate } from '@/react/setup/helpers';
import { useOnboardingStore } from '@/react/setup/store';

import NextButton from '@/react/setup/components/buttons/NextButton';
import TextInput from '../components/TextInput';
import Headline from '../components/headlines/Headline';

const User = () => {
	const navigate = useNavigate();
	const store = useOnboardingStore();

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		if (store.isAccountChecked) {
			e.stopPropagation();
			navigate(`/onboarding/${store.page.next}`);
		}
	};

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Enter' && store.isAccountChecked) {
				e.preventDefault();
				e.stopPropagation();
				navigate(`/onboarding/${store.page.next}`);
			}
		};

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	}, [store.account, store.isAccountChecked]);

	useEffect(() => {
		store.setValid.account({
			email: validate.email(store.account.email),
			username: validate.safe(store.account.username, 5),
			password: validate.len(store.account.password, 8)
		});
	}, [store.account]);

	return (
		<div className="flex flex-col pt-32 md:max-w-xl mx-auto">
			<Headline style="3xl">Create a new account</Headline>

			<div className="mb-4">
				<form className="space-y-3" onSubmit={handleSubmit}>
					<div>
						<label htmlFor="username" className="block text-sm font-medium leading-6 text-zinc-900">
							Username
						</label>
						<div className="mt-1">
							<input
								required
								id="username"
								name="username"
								type="text"
								placeholder="James Bond"
								defaultValue={store.account.username}
								onChange={(e) => store.setAccount.username(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.username && store.account.username && <div className="mt-2 text-sm text-red-500">Please enter a valid username</div>}
						</div>
					</div>

					<div>
						<label htmlFor="email" className="block text-sm font-medium leading-6 text-zinc-900">
							Email address
						</label>
						<div className="mt-1">
							<input
								required
								id="email"
								name="email"
								type="email"
								placeholder="james@bond.com"
								autoComplete="email"
								defaultValue={store.account.email}
								onChange={(e) => store.setAccount.email(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.email && store.account.email && <div className="mt-2 text-sm text-red-500">Please enter a valid email address</div>}
						</div>
					</div>
					<div>
						<label htmlFor="password" className="block text-sm font-medium leading-6 text-zinc-900">
							Password
						</label>
						<div className="mt-1">
							<input
								required
								id="password"
								name="password"
								type="password"
								placeholder="••••••••"
								autoComplete="current-password"
								defaultValue={store.account.password}
								onChange={(e) => store.setAccount.password(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.password && store.account.password && (
								<div className="mt-2 text-sm text-red-500">Password must be longer than 8 characters</div>
							)}
						</div>
					</div>
				</form>
			</div>
			<NextButton className="hidden md:flex" disabled={!store.isAccountChecked} />
			<NextButton className="fixed bottom-0 left-0 md:hidden" disabled={!store.isAccountChecked} />
		</div>
	);
};

export default User;
