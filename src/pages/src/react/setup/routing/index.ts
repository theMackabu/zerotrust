import pagesFile from './pages.json';

const getCurrentPageIndex = (pathname: String): number => {
	const pageName = pathname.split('/')[2];
	const pageIndex = pages.indexOf(pageName);

	return pageIndex;
};

const pages = pagesFile.map((page) => page.slug);

export { pages, getCurrentPageIndex, pagesFile };
