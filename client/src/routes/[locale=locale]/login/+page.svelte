<script lang="ts">
	import { t } from 'svelte-i18n';
	import type { PageData } from './$types';

	let { data } = $props<{ data: PageData }>();

	const canonical = data.seo?.canonical;
	const ogImage = data.seo?.openGraphImage;

	function startLogin() {
		if (!data.loginUrl) {
			return;
		}

		window.location.href = data.loginUrl;
	}
</script>

<svelte:head>
	<title>{$t('login.meta.title')}</title>
	<meta name="description" content={$t('login.meta.description')} />
	{#if canonical}
		<link rel="canonical" href={canonical} />
		<meta property="og:url" content={canonical} />
	{/if}
	<meta property="og:title" content={$t('login.meta.title')} />
	<meta property="og:description" content={$t('login.meta.description')} />
	<meta name="twitter:title" content={$t('login.meta.title')} />
	<meta name="twitter:description" content={$t('login.meta.description')} />
	{#if ogImage}
		<meta property="og:image" content={ogImage} />
		<meta name="twitter:image" content={ogImage} />
	{/if}
</svelte:head>

<section class="login" aria-labelledby="login-heading">
	<div class="login__card">
		<p class="login__eyebrow">{$t('navigation.brand')}</p>
		<h1 id="login-heading">{$t('login.heading')}</h1>
		<p class="login__description">{$t('login.description')}</p>
		<button class="login__cta" on:click={startLogin}>{$t('login.cta')}</button>
		<ul class="login__benefits">
			<li>{$t('home.support.items.verification.title')}</li>
			<li>{$t('home.support.items.guides.title')}</li>
			<li>{$t('home.support.items.backups.title')}</li>
		</ul>
	</div>
</section>

<style>
	.login {
		min-height: 100vh;
		display: grid;
		place-items: center;
		background: radial-gradient(circle at 15% 20%, rgba(124, 58, 237, 0.25), transparent 60%),
			radial-gradient(circle at 80% 0%, rgba(37, 99, 235, 0.35), transparent 50%),
			#0f172a;
		padding: 2.5rem 1.5rem;
	}

	.login__card {
		max-width: 520px;
		width: 100%;
		background: rgba(15, 23, 42, 0.85);
		border: 1px solid rgba(148, 163, 184, 0.2);
		box-shadow: 0 20px 50px rgba(15, 23, 42, 0.45);
		border-radius: 1.5rem;
		padding: clamp(2rem, 5vw, 3rem);
		display: grid;
		gap: 1.25rem;
		text-align: center;
		color: #e2e8f0;
	}

	.login__eyebrow {
		font-size: 0.85rem;
		letter-spacing: 0.3em;
		text-transform: uppercase;
		color: rgba(165, 180, 252, 0.8);
	}

	.login h1 {
		font-size: clamp(1.8rem, 4vw, 2.4rem);
	}

	.login__description {
		color: rgba(226, 232, 240, 0.8);
		line-height: 1.6;
	}

	.login__cta {
		padding: 0.85rem 1.75rem;
		border-radius: 9999px;
		border: none;
		background: linear-gradient(135deg, #7c3aed, #2563eb);
		color: #fff;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 15px 40px rgba(37, 99, 235, 0.35);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.login__cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 20px 50px rgba(124, 58, 237, 0.35);
	}

	.login__benefits {
		list-style: none;
		padding: 0;
		margin: 1rem 0 0;
		display: grid;
		gap: 0.75rem;
		color: rgba(226, 232, 240, 0.75);
	}

	.login__benefits li {
		position: relative;
		padding-left: 1.4rem;
	}

	.login__benefits li::before {
		content: '✔';
		position: absolute;
		left: 0;
		color: rgba(124, 58, 237, 0.9);
	}
</style>

