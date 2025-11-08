import { redirect } from '@sveltejs/kit';
import { DEFAULT_LOCALE } from '$lib/i18n/locales';

export const load = () => {
	throw redirect(302, `/${DEFAULT_LOCALE}`);
};

