import { Link } from 'react-router-dom';
import { ArrowLeftIcon } from '@heroicons/react/24/solid';

export interface SecondaryButtonProps {
	className?: string;
	link: string;
}

export default function SecondaryButton({ link, className }: SecondaryButtonProps) {
	return (
		<Link to={link} className={`flex items-center rounded-full border px-4 py-1 text-sm hover:bg-zinc-200 ${className}`}>
			<ArrowLeftIcon className="mr-2 w-4" />
			Back
		</Link>
	);
}
