import type { LayoutServerLoad } from './$types';
import { DEFAULT_LOCALE, SUPPORTED_LOCALES } from '$lib/i18n/locales';
import type { SiteMeta } from '$lib/types';

const PRODUCTION_ORIGIN = 'https://softblaze.net';

function resolveOrigins(urlOrigin: string | null): { origin: string; canonicalOrigin: string } {
	if (!urlOrigin || urlOrigin === 'null') {
		return {
			origin: PRODUCTION_ORIGIN,
			canonicalOrigin: PRODUCTION_ORIGIN
		};
	}

	const isLocal =
		urlOrigin.startsWith('http://localhost') ||
		urlOrigin.startsWith('http://127.0.0.1') ||
		urlOrigin.includes('://0.0.0.0');

	return {
		origin: urlOrigin,
		canonicalOrigin: isLocal ? PRODUCTION_ORIGIN : urlOrigin
	};
}

export const load: LayoutServerLoad = async ({ url }) => {
	const { origin, canonicalOrigin } = resolveOrigins(url.origin);

	const site: SiteMeta = {
		name: 'Softblaze',
		domain: 'softblaze.net',
		description:
			'Softblaze delivers instant digital activation codes for essential software suites, operating systems, and productivity tools.',
		defaultLocale: DEFAULT_LOCALE,
		supportedLocales: SUPPORTED_LOCALES,
		origin,
		canonicalOrigin,
		tagline: 'Instant digital activation codes for the tools you rely on.'
	};

	return {
		site
	};
};


