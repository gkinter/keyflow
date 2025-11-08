import type { PageServerLoad } from './$types';
import type { SeoMeta } from '$lib/types';

export const load: PageServerLoad = async ({ parent, url }) => {
	const parentData = await parent();

	const site = parentData.site;
	const locale = parentData.locale ?? site?.defaultLocale ?? 'en-US';
	const canonicalOrigin = site?.canonicalOrigin ?? site?.origin ?? 'https://softblaze.net';
	const canonical = parentData.seo?.canonical ?? `${canonicalOrigin}${url.pathname}${url.search}`;

	const seo: SeoMeta = {
		title: 'Softblaze — Instant Digital Activation Codes for Premium Software',
		description:
			'Purchase authentic activation codes with instant delivery. Softblaze covers operating systems, productivity suites, creative software, and security tools for teams of any size.',
		canonical,
		keywords: [
			'software activation codes',
			'digital license keys',
			'instant software delivery',
			'Softblaze marketplace'
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


