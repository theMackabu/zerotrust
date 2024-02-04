import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { validate, classNames } from '@/react/setup/helpers';
import { useOnboardingStore } from '@/react/setup/store';

import NextButton from '@/react/setup/components/buttons/NextButton';
import TextInput from '../components/TextInput';
import Headline from '../components/headlines/Headline';

const Services = () => {
	const navigate = useNavigate();
	const store = useOnboardingStore();

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		if (store.isServicesChecked) {
			e.stopPropagation();
			navigate(`/onboarding/${store.page.next}`);
		}
	};

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Enter' && store.isServicesChecked) {
				e.preventDefault();
				e.stopPropagation();
				navigate(`/onboarding/${store.page.next}`);
			}
		};

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	}, [store.settings, store.isServicesChecked]);

	useEffect(() => {
		store.setValid.services({
			address: validate.url(store.services.address),
			displayName: validate.display(store.services.displayName, 2)
		});
	}, [store.services]);

	return (
		<div className="flex flex-col pt-32 md:max-w-xl mx-auto">
			<Headline style="3xl">Add your first service</Headline>

			<div className="mb-4">
				<form className="space-y-3" onSubmit={handleSubmit}>
					<div>
						<label htmlFor="icon" className="block text-sm font-medium leading-6 text-zinc-900">
							Display Name
						</label>
						<div className="mt-1">
							<input
								required
								id="icon"
								name="icon"
								type="text"
								defaultValue={store.services.displayName}
								placeholder="Example Service"
								onChange={(e) => store.setServices.displayName(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.displayName && store.services.displayName && (
								<div className="mt-2 text-sm text-red-500">Please enter a valid display name</div>
							)}
						</div>
					</div>
					<div>
						<label htmlFor="prefix" className="block text-sm font-medium leading-6 text-zinc-900">
							Service Address
						</label>
						<div className="mt-1">
							<input
								required
								id="prefix"
								name="prefix"
								type="text"
								placeholder="https://example.com"
								defaultValue={store.services.address}
								onChange={(e) => store.setServices.address(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.address && store.services.address && <div className="mt-2 text-sm text-red-500">Please enter a valid address</div>}
						</div>
					</div>
				</form>
			</div>
			<NextButton className="hidden md:flex" disabled={!store.isServicesChecked} />
			<NextButton className="fixed bottom-0 left-0 md:hidden" disabled={!store.isServicesChecked} />
		</div>
	);
};

export default Services;
