import { nanoid } from 'nanoid';
import { create } from 'zustand';
import { pages } from '@/react/setup/routing';

const useOnboardingStore = create((set) => ({
	app: null,

	account: {
		email: '',
		username: '',
		password: ''
	},

	settings: {
		icon: '',
		prefix: '',
		accent: undefined,
		secret: nanoid(30)
	},

	services: {
		address: '',
		displayName: ''
	},

	page: {
		current: 0,
		first: pages[0],
		next: pages[1]
	},

	valid: {
		email: false,
		username: false,
		password: false,
		icon: false,
		prefix: false,
		accent: false,
		displayName: false,
		address: false
	},

	isAccountChecked: false,
	isSettingsChecked: false,
	isServicesChecked: false,

	setApp: (app) => {
		set({ app });
		set((state) => ({ settings: { ...state.settings, icon: app.logo } }));
		set((state) => ({ settings: { ...state.settings, prefix: app.prefix } }));
	},

	setAccount: {
		email: (email) => set((state) => ({ account: { ...state.account, email } })),
		username: (username) => set((state) => ({ account: { ...state.account, username } })),
		password: (password) => set((state) => ({ account: { ...state.account, password } }))
	},

	setSettings: {
		icon: (icon) => set((state) => ({ settings: { ...state.settings, icon } })),
		prefix: (prefix) => set((state) => ({ settings: { ...state.settings, prefix } })),
		accent: (accent) => set((state) => ({ settings: { ...state.settings, accent } })),
		secret: (secret) => set((state) => ({ settings: { ...state.settings, secret } }))
	},

	setServices: {
		address: (address) => set((state) => ({ services: { ...state.services, address } })),
		displayName: (displayName) => set((state) => ({ services: { ...state.services, displayName } }))
	},

	setPage: (index) => {
		set((state) => ({ page: { ...state.page, current: index } }));
		set((state) => ({ page: { ...state.page, next: pages[index + 1] } }));
	},

	setValid: {
		account: (account) => {
			set((state) => ({ ...state.valid, valid: account }));
			set((state) => ({ isAccountChecked: state.valid.email && state.valid.username && state.valid.password }));
		},
		settings: (settings) => {
			set((state) => ({ ...state.valid, valid: settings }));
			set((state) => ({ isSettingsChecked: state.valid.icon && state.valid.prefix && state.valid.accent }));
		},
		services: (services) => {
			set((state) => ({ ...state.valid, valid: services }));
			set((state) => ({ isServicesChecked: state.valid.displayName && state.valid.address }));
		}
	}
}));

export { useOnboardingStore };
