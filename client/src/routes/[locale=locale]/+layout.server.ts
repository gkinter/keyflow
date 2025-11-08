import type { LayoutServerLoad } from './$types';
import type { SessionPayload, SeoMeta } from '$lib/types';
import {
	DEFAULT_LOCALE,
	LOCALE_LABELS,
	SUPPORTED_LOCALES,
	type Locale
} from '$lib/i18n/locales';

const API_BASE = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:8080';

function resolveLocale(value: string | undefined): Locale {
	if (value && SUPPORTED_LOCALES.includes(value as Locale)) {
		return value as Locale;
	}

	return DEFAULT_LOCALE;
}

function buildSuffix(pathname: string): string {
	const [, ...rest] = pathname.split('/').filter(Boolean);

	if (rest.length === 0) {
		return '';
	}

	return `/${rest.join('/')}`;
}

function buildAlternates(canonicalOrigin: string, suffix: string): Record<Locale, string> {
	return SUPPORTED_LOCALES.reduce((acc, locale) => {
		acc[locale] = `${canonicalOrigin}/${locale}${suffix}`;
		return acc;
	}, {} as Record<Locale, string>);
}

export const load: LayoutServerLoad = async ({ params, fetch, url, parent, setHeaders }) => {
	const parentData = await parent();

	const locale = resolveLocale(params.locale);
	const suffix = buildSuffix(url.pathname);

	setHeaders({
		'content-language': locale
	});

	let session: SessionPayload | null = null;

	try {
		const response = await fetch(`${API_BASE}/session`, {
			credentials: 'include'
		});

		if (response.ok) {
			session = (await response.json()) as SessionPayload;
		}
	} catch (error) {
		console.warn('Failed to load session', error);
	}

	const canonicalOrigin =
		parentData.site?.canonicalOrigin ?? parentData.site?.origin ?? 'https://softblaze.net';

	const canonical = `${canonicalOrigin}/${locale}${suffix}${url.search}`;
	const alternates = buildAlternates(canonicalOrigin, suffix);

	const seo: SeoMeta = {
		title: parentData.site
			? `${parentData.site.name} — ${parentData.site.tagline}`
			: 'Softblaze — Digital Activation Codes',
		description:
			parentData.site?.description ??
			'Softblaze delivers instant digital activation codes for essential software suites and productivity tools.',
		canonical,
		keywords: [
			'digital software licenses',
			'software activation codes',
			'instant delivery license keys',
			'Softblaze'
		],
		openGraphImage: `${canonicalOrigin}/og-softblaze.svg`,
		locale,
		alternates
	};

	return {
		session,
		loginUrl: `${API_BASE}/auth/github/login`,
		locale,
		availableLocales: SUPPORTED_LOCALES,
		localeLabels: LOCALE_LABELS,
		seo
	};
};


