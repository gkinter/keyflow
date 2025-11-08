export const SUPPORTED_LOCALES = [
	'en-US',
	'en-GB',
	'fr-FR',
	'de-DE',
	'es-ES',
	'it-IT',
	'ja-JP',
	'zh-CN',
	'zh-TW',
	'ko-KR'
] as const;

export type Locale = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: Locale = 'en-US';

export const LOCALE_LABELS: Record<Locale, string> = {
	'en-US': 'English (United States)',
	'en-GB': 'English (United Kingdom)',
	'fr-FR': 'Français',
	'de-DE': 'Deutsch',
	'es-ES': 'Español',
	'it-IT': 'Italiano',
	'ja-JP': '日本語',
	'zh-CN': '简体中文',
	'zh-TW': '繁體中文',
	'ko-KR': '한국어'
};

export function isLocale(value: string): value is Locale {
	return SUPPORTED_LOCALES.some((locale) => locale === value);
}

