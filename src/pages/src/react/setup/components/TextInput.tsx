export interface TextInputProps {
	placeholder?: string;
	type?: string;
	defaultValue?: string;
	autoFocus?: boolean;
	onChange: (value: string) => void;
}

export default function TextInput({ onChange, placeholder = '', type = 'text', defaultValue = '', autoFocus = true }: TextInputProps) {
	const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		onChange(e.target.value);
	};

	return (
		<input
			defaultValue={defaultValue}
			autoFocus={autoFocus}
			type={type}
			placeholder={placeholder}
			className="block w-full rounded-md border-0 px-4 py-1.5 text-zinc-900 
        shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 
        focus:ring-2 focus:ring-inset focus:ring-indigo-700 sm:text-sm sm:leading-6"
			onChange={handleChange}
		/>
	);
}
