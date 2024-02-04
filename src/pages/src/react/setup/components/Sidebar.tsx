import { Link } from 'react-router-dom';
import { CheckCircleIcon } from '@heroicons/react/20/solid';
import { pagesFile } from '@/react/setup/routing';
import { useOnboardingStore } from '@/react/setup/store';

const Sidebar = () => {
	const { page } = useOnboardingStore();
	const onboardingStore = useOnboardingStore();

	const isStepChecked = (stepIndex: number) => {
		const step = pagesFile[stepIndex];
		const storeKey = step.storeKey;

		return storeKey && onboardingStore[storeKey];
	};

	const isStepCurrent = (stepIndex: number) => page.current === stepIndex;
	const isStepActive = (stepIndex: number) => !isStepChecked(stepIndex) && page.current === stepIndex;

	return (
		<div className="hidden h-screen bg-indigo-50 md:block border-r border-indigo-200">
			<div className="mb-32 px-8 py-7 font-bold text-xl">
				<Link to="/">Zerotrust</Link>
			</div>

			<div className="px-4 py-12 sm:px-6 lg:px-8">
				<nav className="flex justify-center" aria-label="Progress">
					<ol role="list" className="space-y-6">
						{pagesFile.map((step, index) => (
							<li key={step.slug}>
								{isStepActive(index) && (
									<div className="flex items-start cursor-default">
										<span className="relative flex h-5 w-5 shrink-0 items-center justify-center" aria-hidden="true">
											<span className="absolute h-4 w-4 rounded-full bg-indigo-300/50" />
											<span className="relative block h-2 w-2 rounded-full bg-indigo-700" />
										</span>
										<span className="ml-3 text-sm font-medium text-indigo-700">{step.name}</span>
									</div>
								)}

								{isStepChecked(index) && (
									<Link to={step.slug} className="group">
										<span className="flex items-start">
											<span className="relative flex h-5 w-5 shrink-0 items-center justify-center">
												<CheckCircleIcon className={`h-full w-full text-indigo-700`} aria-hidden="true" />
											</span>
											<span className={`ml-3 text-sm font-medium ${isStepCurrent(index) ? 'text-indigo-600' : 'text-zinc-500'}`}>{step.name}</span>
										</span>
									</Link>
								)}

								{!isStepActive(index) && !isStepChecked(index) && (
									<div className="flex items-start cursor-default">
										<div className="relative flex h-5 w-5 shrink-0 items-center justify-center" aria-hidden="true">
											<div className="h-2 w-2 rounded-full bg-zinc-400" />
										</div>
										<p className="ml-3 text-sm font-medium text-zinc-500">{step.name}</p>
									</div>
								)}
							</li>
						))}
					</ol>
				</nav>
			</div>
		</div>
	);
};

export default Sidebar;
