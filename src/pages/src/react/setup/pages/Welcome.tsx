import { useEffect } from 'react';
import { pagesFile } from '@/react/setup/routing';
import { useNavigate, Link } from 'react-router-dom';
import { useOnboardingStore } from '@/react/setup/store';

const Welcome = () => {
	const navigate = useNavigate();
	const { page } = useOnboardingStore();

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Enter') {
				e.preventDefault();
				e.stopPropagation();
				navigate(`/onboarding/${page.first}`);
			}
		};

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	}, [page.first, navigate]);

	return (
		<div class="grid grid-cols-1 h-screen place-items-center">
			<div class="sm:mx-auto w-full sm:max-w-2xl p-4 sm:p-12 justify-center">
				<div className="flex flex-col items-center justify-center">
					<div className="text-4xl sm:text-5xl font-bold mb-5">Welcome to Zerotrust</div>
					<div className="mb-12 text-zinc-600 text-xl">Create the default user, and your first service to continue</div>

					<div className="flex max-w-xl flex-col items-center justify-center">
						<Link
							to={`/onboarding/${pagesFile[0].slug}`}
							className="bg-indigo-600 hover:bg-indigo-500 border border-indigo-700 transition text-white hover:scale-[1.02] rounded-md font-bold mb-4 px-10 py-4 text-xl md:w-auto">
							Get started
						</Link>

						<div className="text-xs text-zinc-500">
							or press <kbd className="px-1 py-0.5 mr-1 text-xs font-semibold text-zinc-600 bg-zinc-50 border border-zinc-100 rounded">ENTER</kbd>
							to start setup
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};

export default Welcome;
