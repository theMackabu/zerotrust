import { Transition } from '@headlessui/react';
import { Onboarding } from '@/react/setup/components';
import { useEffect, Fragment, ChangeEvent } from 'react';
import { useOnboardingStore } from '@/react/setup/store';
import { getCurrentPageIndex } from '@/react/setup/routing';
import { useLocation, MemoryRouter, Routes, Route } from 'react-router-dom';
import { Welcome, User, Settings, Services, Summary } from '@/react/setup/pages';

const Setup = (props: { app }) => {
	const location = useLocation();
	const { setPage, setApp } = useOnboardingStore();

	useEffect(() => setApp(props.app), []);
	useEffect(() => setPage(getCurrentPageIndex(location.pathname)), [location]);

	return (
		<Routes>
			<Route path="/" element={<Welcome />} />
			<Route path="/onboarding" element={<Onboarding />}>
				<Route path="user" element={<User />} />
				<Route path="settings" element={<Settings />} />
				<Route path="services" element={<Services />} />
				<Route path="summary" element={<Summary />} />
			</Route>
		</Routes>
	);
};

const Index = (props: { app }) => (
	<MemoryRouter>
		<Setup app={props.app} />
	</MemoryRouter>
);

export default Index;
