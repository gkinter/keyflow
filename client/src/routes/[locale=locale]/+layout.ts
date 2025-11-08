import type { LayoutLoad } from './$types';
import type { SessionPayload } from '$lib/types';
import { isSupportedLocale, loadLocale } from '$lib/i18n';
import { DEFAULT_LOCALE, LOCALE_LABELS, SUPPORTED_LOCALES, type Locale } from '$lib/i18n/locales';

const API_BASE: string = (import.meta as any).env?.VITE_API_BASE_URL ?? '';

export const ssr = false;

export const load: LayoutLoad = async ({ params, fetch, data }) => {
	const requestedLocale = params.locale ?? data?.locale ?? DEFAULT_LOCALE;
	const locale = (isSupportedLocale(requestedLocale) ? requestedLocale : DEFAULT_LOCALE) as Locale;

	await loadLocale(locale);

	let session: SessionPayload | null = null;

	if (API_BASE) {
		try {
			const response = await fetch(`${API_BASE}/session`, {
				credentials: 'include'
			});

			if (response.ok) {
				session = await response.json();
			}
		} catch (error) {
			console.warn('Failed to load session', error);
		}
	}

	return {
		session,
		loginUrl: API_BASE ? `${API_BASE}/auth/github/login` : '/auth/github/login',
		locale,
		availableLocales: SUPPORTED_LOCALES,
		localeLabels: LOCALE_LABELS
	};
};
