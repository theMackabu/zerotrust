import { useState, useEffect, Fragment } from 'react';
import { useNavigate } from 'react-router-dom';
import { validate, classNames } from '@/react/setup/helpers';
import { useOnboardingStore } from '@/react/setup/store';

import { Listbox, Transition } from '@headlessui/react';
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/react/20/solid';

import NextButton from '@/react/setup/components/buttons/NextButton';
import TextInput from '../components/TextInput';
import Headline from '../components/headlines/Headline';

const colors = [
	'red',
	'orange',
	'amber',
	'yellow',
	'lime',
	'green',
	'emerald',
	'teal',
	'cyan',
	'sky',
	'blue',
	'indigo',
	'violet',
	'purple',
	'fuchsia',
	'pink',
	'rose'
];

const Settings = () => {
	const navigate = useNavigate();
	const store = useOnboardingStore();
	const [selected, setSelected] = useState(store.settings.accent);

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		if (store.isSettingsChecked) {
			e.stopPropagation();
			navigate(`/onboarding/${store.page.next}`);
		}
	};

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Enter' && store.isSettingsChecked) {
				e.preventDefault();
				e.stopPropagation();
				navigate(`/onboarding/${store.page.next}`);
			}
		};

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	}, [store.settings, store.isSettingsChecked]);

	useEffect(() => {
		store.setSettings.accent(selected);
	}, [selected]);

	useEffect(() => {
		store.setValid.settings({
			accent: selected != undefined,
			icon: validate.safe(store.settings.icon, 3),
			prefix: validate.safe(store.settings.prefix, 1)
		});
	}, [store.settings]);

	return (
		<div className="flex flex-col pt-32 md:max-w-xl mx-auto">
			<Headline style="3xl">Change some defaults</Headline>

			<div className="mb-4">
				<form className="space-y-3" onSubmit={handleSubmit}>
					<div>
						<label htmlFor="icon" className="block text-sm font-medium leading-6 text-zinc-900">
							Icon URL
						</label>
						<div className="mt-1">
							<input
								required
								id="icon"
								name="icon"
								type="text"
								placeholder={store.app.logo}
								defaultValue={store.settings.icon}
								onChange={(e) => store.setSettings.icon(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.icon && store.settings.icon && <div className="mt-2 text-sm text-red-500">Please enter a valid icon url</div>}
						</div>
					</div>
					<div>
						<Listbox value={selected} onChange={setSelected}>
							{({ open }) => (
								<>
									<Listbox.Label className="block text-sm font-medium leading-6 text-gray-900">Accent color</Listbox.Label>
									<div className="relative mt-2">
										<Listbox.Button className="relative w-full cursor-default rounded-md bg-white py-1.5 pl-3 pr-10 text-left text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6">
											<span className="block truncate capitalize">{selected ? selected : 'Select'}</span>
											<span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
												<ChevronUpDownIcon className="h-5 w-5 text-gray-400" aria-hidden="true" />
											</span>
										</Listbox.Button>

										<Transition show={open} as={Fragment} leave="transition ease-in duration-100" leaveFrom="opacity-100" leaveTo="opacity-0">
											<Listbox.Options className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
												{colors.map((item) => (
													<Listbox.Option
														key={item}
														className={({ active }) =>
															classNames(active ? 'bg-indigo-600 text-white' : 'text-gray-900', 'relative cursor-default select-none py-2 pl-3 pr-9')
														}
														value={item}>
														{({ selected, active }) => (
															<>
																<span className={classNames(selected ? 'font-semibold' : 'font-normal', 'block truncate capitalize')}>{item}</span>

																{selected ? (
																	<span
																		className={classNames(
																			active ? 'text-white' : 'text-indigo-600',
																			'absolute inset-y-0 right-0 flex items-center pr-4'
																		)}>
																		<CheckIcon className="h-5 w-5" aria-hidden="true" />
																	</span>
																) : null}
															</>
														)}
													</Listbox.Option>
												))}
											</Listbox.Options>
										</Transition>
									</div>
								</>
							)}
						</Listbox>
					</div>
					<div>
						<label htmlFor="prefix" className="block text-sm font-medium leading-6 text-zinc-900">
							Internal Prefix
						</label>
						<div className="mt-1">
							<input
								required
								id="prefix"
								name="prefix"
								type="text"
								placeholder={store.app.prefix}
								defaultValue={store.settings.prefix}
								onChange={(e) => store.setSettings.prefix(e.target.value)}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
							{!store.valid.prefix && store.settings.prefix && <div className="mt-2 text-sm text-red-500">Please enter a valid prefix</div>}
						</div>
					</div>
					<div>
						<label htmlFor="secret" className="block text-sm font-medium leading-6 text-zinc-900">
							Database Secret
						</label>
						<div className="mt-1">
							<input
								required
								id="secret"
								name="secret"
								type="text"
								disabled={true}
								defaultValue={store.settings.secret}
								className={`transition block w-full rounded-md border-0 py-1.5 text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:opacity-70`}
							/>
						</div>
					</div>
				</form>
			</div>
			<NextButton className="hidden md:flex" disabled={!store.isSettingsChecked} />
			<NextButton className="fixed bottom-0 left-0 md:hidden" disabled={!store.isSettingsChecked} />
		</div>
	);
};

export default Settings;
