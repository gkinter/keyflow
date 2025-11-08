import type { PageServerLoad } from './$types';
import type { SeoMeta } from '$lib/types';

export const load: PageServerLoad = async ({ parent, url }) => {
	const parentData = await parent();

	const site = parentData.site;
	const locale = parentData.locale ?? site?.defaultLocale ?? 'en-US';
	const canonicalOrigin = site?.canonicalOrigin ?? site?.origin ?? 'https://softblaze.net';
	const canonical = parentData.seo?.canonical ?? `${canonicalOrigin}${url.pathname}${url.search}`;

	const seo: SeoMeta = {
		title: 'Sign in to Softblaze — Access Your Digital Licenses',
		description:
			'Log into Softblaze to download activation codes, review invoices, and manage software licenses from one secure dashboard.',
		canonical,
		keywords: [
			'Softblaze login',
			'software license dashboard',
			'digital activation code management',
			'software license invoices'
		],
		openGraphImage:
			parentData.seo?.openGraphImage ?? `${canonicalOrigin}/og-softblaze.svg`,
		locale: parentData.seo?.locale ?? locale,
		alternates: parentData.seo?.alternates
	};

	return {
		seo
	};
};


