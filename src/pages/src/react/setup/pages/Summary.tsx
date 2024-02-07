import { pagesFile } from '@/react/setup/routing';
import { useNavigate, Link } from 'react-router-dom';
import { useEffect, useState, Fragment } from 'react';
import Headline from '../components/headlines/Headline';
import { useOnboardingStore } from '@/react/setup/store';

const Summary = () => {
	const navigate = useNavigate();
	const store = useOnboardingStore();
	const [loading, setLoading] = useState(false);

	const validityStoreKeys = pagesFile.filter((step) => step.storeKey).map((step) => step.storeKey);
	const submitButtonEnabled = validityStoreKeys.every((storeKey) => store[storeKey]);

	async function loginNewUser() {
		await new Promise((r) => setTimeout(r, 5000));
		fetch(`/${store.settings.prefix}/api/login`, {
			method: 'POST',
			body: JSON.stringify({
				remember: true,
				email: store.account.email,
				password: store.account.password
			}),
			headers: { 'Content-Type': 'application/json' }
		}).then(async (response) => {
			if (response.status === 200) {
				window.location.href = `/${store.settings.prefix}/app`;
			}
		});
	}

	console.log(store);

	const submitSetupData = () => {
		setLoading(true);
		fetch('/setup', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				account: store.account,
				settings: store.settings,
				service: !store.services.skipped
					? () => {
							const serviceAddress = new URL(store.services.address);
							return {
								name: store.services.displayName.trim().replace(' ', '_'),
								display: store.services.displayName,
								address: serviceAddress.hostname,
								port: parseInt(serviceAddress.port),
								tls: serviceAddress.protocol === 'https:'
							};
						}
					: undefined
			})
		})
			.then(() => loginNewUser())
			.catch(() => loginNewUser());
	};

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Enter' && submitButtonEnabled) {
				e.preventDefault();
				e.stopPropagation();
				submitSetupData();
			}
		};

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	}, [navigate, submitButtonEnabled, store]);

	return (
		<div className="flex flex-col pt-20 md:max-w-xl mx-auto">
			<Headline style="3xl">Is this information correct?</Headline>
			<div className="mt-2 block text-sm font-medium leading-6 text-zinc-900">Default Account</div>
			<div className="mt-0.5 grid grid-cols-[1fr,2fr] gap-1 rounded-md border-[1px] border-dashed border-zinc-300">
				<div className="px-4 py-2">Username:</div>
				<div className="px-4 py-2">{store.account.username}</div>
				<div className="px-4 py-2">Email:</div>
				<div className="px-4 py-2">{store.account.email}</div>
			</div>
			<div className="mt-4 block text-sm font-medium leading-6 text-zinc-900">App Settings</div>
			<div className="mt-0.5 grid grid-cols-[1fr,2fr] gap-1 rounded-md border-[1px] border-dashed border-zinc-300">
				<div className="px-4 py-2">Prefix:</div>
				<div className="px-4 py-2">{store.settings.prefix}</div>
				<div className="px-4 py-2">Icon URL:</div>
				<div className="px-4 py-2">{store.settings.icon}</div>
			</div>
			{!store.services.skipped ? (
				<Fragment>
					<div className="mt-4 block text-sm font-medium leading-6 text-zinc-900">New Service</div>
					<div className="mt-0.5 mb-12 grid grid-cols-[1fr,2fr] gap-1 rounded-md border-[1px] border-dashed border-zinc-300">
						<div className="px-4 py-2">Name:</div>
						<div className="px-4 py-2">{store.services.displayName}</div>
						<div className="px-4 py-2">Address:</div>
						<div className="px-4 py-2">{store.services.address}</div>
					</div>
				</Fragment>
			) : (
				<div className="mb-12" />
			)}
			<div className="fixed bottom-0 left-0 w-full px-8 py-4 md:relative md:p-0">
				<button
					className="bg-indigo-600 hover:bg-indigo-500 border border-indigo-700 transition text-white rounded-md font-bold px-8 py-4 text-lg md:w-auto disabled:pointer-events-none disabled:opacity-80"
					onClick={submitSetupData}
					disabled={!submitButtonEnabled || loading}>
					{loading ? 'Saving settings...' : 'Continue to dashboard'}
				</button>
			</div>
		</div>
	);
};

export default Summary;
