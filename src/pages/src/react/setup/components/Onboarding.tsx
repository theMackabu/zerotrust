import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import { useEffect, useState } from 'react';
import { useOnboardingStore } from '@/react/setup/store';
import { pagesFile } from '@/react/setup/routing';
import NextButton from './buttons/NextButton';
import { pages } from '@/react/setup/routing';
import SecondaryButton from './buttons/SecondaryButton';

const Onboarding = () => {
	const { page } = useOnboardingStore();
	const [previousPage, setPreviousPage] = useState(pages[page.current - 1]);

	useEffect(() => {
		setPreviousPage(pages[page.current - 1]);
	}, [page.current]);

	return (
		<div className="col-auto grid	p-8 md:h-screen md:grid-cols-[1fr,3fr] md:p-0">
			<aside className="hidden md:block">
				<Sidebar />
			</aside>
			<div>
				<nav className="flex h-20 max-h-20 w-full items-center justify-end">
					<div className="flex items-center">
						{page.current > 0 && <SecondaryButton className="mr-5" link={`/onboarding/${previousPage}`} />}
						<div className="mr-6 leading-none text-neutral-500 md:hidden">
							{page.current + 1} / {pages.length}
						</div>
					</div>
				</nav>
				<div className="px-0 md:px-16">
					<Outlet />
				</div>
			</div>
		</div>
	);
};

export default Onboarding;
