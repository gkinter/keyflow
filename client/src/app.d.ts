// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		interface PageData {
			session: import('./lib/types').SessionPayload | null;
			loginUrl: string;
			locale?: import('./lib/i18n/locales').Locale;
			availableLocales?: readonly import('./lib/i18n/locales').Locale[];
			localeLabels?: Record<import('./lib/i18n/locales').Locale, string>;
			site?: import('./lib/types').SiteMeta;
			seo?: import('./lib/types').SeoMeta;
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
