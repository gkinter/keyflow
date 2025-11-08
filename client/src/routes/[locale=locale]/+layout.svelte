<script lang="ts">
	import '../../app.css';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import type { LayoutData } from './$types';
	import type { Locale } from '$lib/i18n/locales';
	import type { SeoMeta, SessionPayload } from '$lib/types';
	import { t } from 'svelte-i18n';
	import type { SvelteSlots } from 'svelte';

	let { data, children } = $props<{ data: LayoutData; children: SvelteSlots }>();

	let session = $state<SessionPayload | null>(data.session);
	let seo = $state<SeoMeta | undefined>(data.seo);

	$effect(() => {
		session = data.session;
		seo = $page.data?.seo ?? data.seo;
	});

	const site = data.site;
	const locale = data.locale ?? site?.defaultLocale ?? 'en-US';
	const availableLocales = data.availableLocales ?? site?.supportedLocales ?? [];
	const localeLabels = data.localeLabels ?? {};

	const suffix = $derived(() => {
		const segments = $page.url.pathname.split('/').slice(2).filter(Boolean);
		return segments.length ? `/${segments.join('/')}` : '';
	});

	const canonical = $derived(() => {
		if (seo?.canonical) {
			return seo.canonical;
		}

		const origin = site?.canonicalOrigin ?? site?.origin;
		return origin ? `${origin}/${locale}${suffix}` : undefined;
	});

	const keywords = $derived(() =>
		seo?.keywords && seo.keywords.length ? seo.keywords.join(', ') : undefined
	);
	const alternates = $derived(() => seo?.alternates ?? {});
	const alternateEntries = $derived(() =>
		Object.entries(alternates).filter(([code]) => code !== locale)
	);
	const ogImage = $derived(() => seo?.openGraphImage ?? `${site?.canonicalOrigin ?? site?.origin ?? ''}/og-softblaze.svg`);

	function toOgLocale(code: string): string {
		return code.replace('-', '_');
	}

	$effect(() => {
		if (browser) {
			document.documentElement.lang = locale;
			document.body.dataset.locale = locale;
		}
	});

	const apiBase = $derived(() => {
		if (data.loginUrl) {
			try {
				return new URL(data.loginUrl).origin;
			} catch {
				// ignore parse failures
			}
		}

		return site?.origin ?? '';
	});

	function buildLocaleHref(targetLocale: Locale) {
		const current = $page;
		const segments = current.url.pathname.split('/').slice(2).filter(Boolean);
		const path = segments.length ? `/${segments.join('/')}` : '';
		const search = current.url.search ?? '';
		const hash = current.url.hash ?? '';

		return `/${targetLocale}${path}${search}${hash}`;
	}

	async function handleLogout() {
		if (!session) {
			goto(`/${locale}`);
			return;
		}

		try {
			await fetch(`${apiBase}/auth/logout`, {
				method: 'POST',
				credentials: 'include',
				headers: {
					'X-CSRF-Token': session.csrfToken
				}
			});
		} finally {
			session = null;
			goto(`/${locale}`);
		}
	}

	function startLogin() {
		if (!data.loginUrl) {
			return;
		}

		window.location.href = data.loginUrl;
	}

	const isPublicRoute = $derived(() => {
		const current = $page;
		const segments = current.url.pathname.split('/').slice(2).filter(Boolean);
		return segments.length === 0 || segments[0] === 'login';
	});

	const organizationJsonLd = $derived(() => {
		if (!site || !canonical) {
			return null;
		}

		return {
			'@context': 'https://schema.org',
			'@type': 'Store',
			name: site.name,
			url: canonical,
			description: seo?.description ?? site.description,
			image: ogImage,
			areaServed: 'Worldwide',
			brand: {
				'@type': 'Brand',
				name: site.name,
				url: site.canonicalOrigin
			},
			knowsAbout: [
				'software activation codes',
				'digital license keys',
				'operating system keys',
				'productivity suites',
				'security software'
			],
			sameAs: [`https://${site.domain}`]
		};
	});
</script>

<svelte:head>
	<title>{seo?.title ?? site?.name ?? 'Softblaze'}</title>
	{#if seo?.description ?? site?.description}
		<meta name="description" content={seo?.description ?? site?.description} />
	{/if}
	{#if canonical}
		<link rel="canonical" href={canonical} />
		<meta property="og:url" content={canonical} />
	{/if}
	{#if seo?.locale}
		<meta property="og:locale" content={toOgLocale(seo.locale)} />
	{/if}
	<meta property="og:title" content={seo?.title ?? site?.name ?? 'Softblaze'} />
	{#if seo?.description ?? site?.description}
		<meta property="og:description" content={seo?.description ?? site?.description} />
	{/if}
	{#if ogImage}
		<meta property="og:image" content={ogImage} />
		<meta name="twitter:image" content={ogImage} />
	{/if}
	<meta property="og:type" content="website" />
	<meta name="robots" content="index,follow" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content={seo?.title ?? site?.name ?? 'Softblaze'} />
	{#if seo?.description ?? site?.description}
		<meta name="twitter:description" content={seo?.description ?? site?.description} />
	{/if}
	{#if keywords}
		<meta name="keywords" content={keywords} />
	{/if}
	{#if alternateEntries.length}
		{#each alternateEntries as [code, href]}
			<link rel="alternate" hrefLang={code} href={href} />
			<meta property="og:locale:alternate" content={toOgLocale(code)} />
		{/each}
	{/if}
	{#if site}
		<link
			rel="alternate"
			hrefLang="x-default"
			href={`${site.canonicalOrigin}/${site.defaultLocale}${suffix}`}
		/>
	{/if}
	{#if organizationJsonLd}
		<script type="application/ld+json">{JSON.stringify(organizationJsonLd)}</script>
	{/if}
</svelte:head>

{#if session}
	<div class="app-shell">
		<header class="app-shell__header" aria-label="Softblaze primary navigation">
			<div class="app-shell__brand" aria-label={$t('navigation.brand')}>
				{$t('navigation.brand')}
			</div>
			<nav class="app-shell__locale" aria-label={$t('navigation.localeLabel')}>
				<span>{$t('navigation.localeLabel')}</span>
				<div class="app-shell__locale-options">
					{#each availableLocales as code}
						<a href={buildLocaleHref(code)} class:active={code === locale}>
							{localeLabels[code] ?? code}
						</a>
					{/each}
				</div>
			</nav>
			<div class="app-shell__user">
				{#if session.user.avatar_url}
					<img src={session.user.avatar_url} alt="" class="app-shell__avatar" />
				{/if}
				<span class="app-shell__username">{session.user.login}</span>
				<button class="app-shell__logout" on:click={handleLogout}>
					{$t('navigation.logout')}
				</button>
			</div>
		</header>
		<main class="app-shell__content" role="main">
			{@render children()}
		</main>
	</div>
{:else if isPublicRoute}
	{@render children()}
{:else}
	<div class="auth-gate">
		<h1>{$t('auth.welcomeBackTitle')}</h1>
		<p>{$t('auth.welcomeBackDescription')}</p>
		<button class="auth-gate__login" on:click={startLogin}>
			{$t('auth.loginCta')}
		</button>
	</div>
{/if}

<style>
	.app-shell {
		display: grid;
		grid-template-rows: auto 1fr;
		min-height: 100vh;
		background-color: var(--color-background, #0f172a);
		color: #f8fafc;
	}

	.app-shell__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
		background: rgba(15, 23, 42, 0.9);
		backdrop-filter: blur(6px);
	}

	.app-shell__brand {
		font-weight: 600;
		font-size: 1.25rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	.app-shell__locale {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.9rem;
		color: #cbd5f5;
	}

	.app-shell__locale-options {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.app-shell__locale-options a {
		color: inherit;
		text-decoration: none;
		padding: 0.25rem 0.5rem;
		border-radius: 9999px;
		border: 1px solid transparent;
		transition: border-color 0.2s ease, background 0.2s ease;
	}

	.app-shell__locale-options a:hover {
		border-color: rgba(148, 163, 184, 0.6);
	}

	.app-shell__locale-options a.active {
		border-color: rgba(148, 163, 184, 0.8);
		background: rgba(148, 163, 184, 0.15);
	}

	.app-shell__user {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.app-shell__avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		object-fit: cover;
	}

	.app-shell__username {
		font-weight: 500;
	}

	.app-shell__logout {
		padding: 0.4rem 0.85rem;
		border-radius: 0.5rem;
		border: 1px solid rgba(148, 163, 184, 0.4);
		background: transparent;
		color: inherit;
		cursor: pointer;
		transition: background 0.2s ease, transform 0.2s ease;
	}

	.app-shell__logout:hover {
		background: rgba(148, 163, 184, 0.1);
		transform: translateY(-1px);
	}

	.app-shell__content {
		padding: 1.5rem;
	}

	.auth-gate {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		text-align: center;
		background: radial-gradient(circle at top, #1e293b, #0f172a);
		color: #e2e8f0;
		padding: 2rem;
	}

	.auth-gate__login {
		padding: 0.75rem 1.5rem;
		border-radius: 9999px;
		border: none;
		background: linear-gradient(135deg, #2563eb, #7c3aed);
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 12px 30px rgba(37, 99, 235, 0.25);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.auth-gate__login:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 35px rgba(124, 58, 237, 0.3);
	}
</style>

