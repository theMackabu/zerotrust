import { Link } from 'react-router-dom';
import { useOnboardingStore } from '../../store';
import ArrowRightIcon from '../ArrowRightIcon';

export type NextButtonProps = {
	className?: string;
	disabled: boolean;
};

export default function NextButton({ className = '', disabled, buttonText = 'Next', onClick = () => {} }: NextButtonProps) {
	const { page } = useOnboardingStore();

	return (
		<div className={`flex w-full flex-row justify-between gap-4 px-8 py-4 md:p-0 ${className}`}>
			{page.next && (
				<div className="w-full md:ml-auto md:w-auto">
					<Link
						onClick={onClick}
						to={`/onboarding/${page.next}`}
						className={`flex items-center justify-center whitespace-nowrap 
              rounded-md px-6 py-3 text-base font-medium text-white 
              ${
								disabled
									? 'pointer-events-none bg-neutral-400'
									: 'bg-neutral-900 transition-colors duration-500 focus:outline-none md:hover:bg-neutral-700'
							}
            `}>
						{buttonText}
						<ArrowRightIcon className="ml-2" />
					</Link>
				</div>
			)}
		</div>
	);
}
