import { addMessages, init, locale as localeStore, waitLocale } from 'svelte-i18n';
import { DEFAULT_LOCALE, SUPPORTED_LOCALES, type Locale } from './locales';
import enUS from './translations/en-US.json';

SUPPORTED_LOCALES.forEach((loc) => {
	addMessages(loc, enUS);
});

let initialized = false;

export async function loadLocale(locale: Locale) {
	if (!initialized) {
		init({
			fallbackLocale: DEFAULT_LOCALE,
			initialLocale: locale
		});
		initialized = true;
	} else {
		localeStore.set(locale);
	}

	await waitLocale();
}

export function isSupportedLocale(value: string): value is Locale {
	return SUPPORTED_LOCALES.some((locale) => locale === value);
}

