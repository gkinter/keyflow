<script lang="ts">
	import { t } from 'svelte-i18n';
	import type { PageData } from './$types';

	let { data } = $props<{ data: PageData }>();

	const featureKeys = [
		{
			title: 'home.features.instantDelivery.title',
			description: 'home.features.instantDelivery.description'
		},
		{
			title: 'home.features.authenticLicenses.title',
			description: 'home.features.authenticLicenses.description'
		},
		{
			title: 'home.features.globalCatalog.title',
			description: 'home.features.globalCatalog.description'
		},
		{
			title: 'home.features.support.title',
			description: 'home.features.support.description'
		}
	];

	const categoryKeys = [
		{
			title: 'home.categories.operatingSystems.title',
			description: 'home.categories.operatingSystems.description'
		},
		{
			title: 'home.categories.productivity.title',
			description: 'home.categories.productivity.description'
		},
		{
			title: 'home.categories.creative.title',
			description: 'home.categories.creative.description'
		},
		{
			title: 'home.categories.security.title',
			description: 'home.categories.security.description'
		}
	];

	const faqKeys = [
		{
			question: 'home.faq.delivery.question',
			answer: 'home.faq.delivery.answer'
		},
		{
			question: 'home.faq.authenticity.question',
			answer: 'home.faq.authenticity.answer'
		},
		{
			question: 'home.faq.installation.question',
			answer: 'home.faq.installation.answer'
		},
		{
			question: 'home.faq.support.question',
			answer: 'home.faq.support.answer'
		}
	];

	const canonical = data.seo?.canonical;
	const ogImage = data.seo?.openGraphImage;
</script>

<svelte:head>
	<title>{$t('home.meta.title')}</title>
	<meta name="description" content={$t('home.meta.description')} />
	{#if canonical}
		<meta property="og:url" content={canonical} />
		<link rel="canonical" href={canonical} />
	{/if}
	<meta property="og:title" content={$t('home.meta.title')} />
	<meta property="og:description" content={$t('home.meta.description')} />
	<meta name="twitter:title" content={$t('home.meta.title')} />
	<meta name="twitter:description" content={$t('home.meta.description')} />
	{#if ogImage}
		<meta property="og:image" content={ogImage} />
		<meta name="twitter:image" content={ogImage} />
	{/if}
</svelte:head>

<section class="hero" aria-labelledby="hero-heading">
	<div class="hero__content">
		<p class="hero__eyebrow">{$t('home.hero.eyebrow')}</p>
		<h1 id="hero-heading">{$t('home.hero.heading')}</h1>
		<p class="hero__description">{$t('home.hero.description')}</p>
		<div class="hero__actions">
			<a class="hero__cta hero__cta--primary" href="#catalog">
				{$t('home.hero.primaryCta')}
			</a>
			<a class="hero__cta hero__cta--secondary" href={data.loginUrl}>
				{$t('home.hero.secondaryCta')}
			</a>
		</div>
		<ul class="hero__trust">
			<li>{$t('home.hero.badges.microsoft')}</li>
			<li>{$t('home.hero.badges.adobe')}</li>
			<li>{$t('home.hero.badges.autodesk')}</li>
			<li>{$t('home.hero.badges.malwarebytes')}</li>
		</ul>
	</div>
</section>

<section class="features" aria-labelledby="features-heading">
	<div class="section-heading">
		<h2 id="features-heading">{$t('home.features.heading')}</h2>
		<p>{$t('home.features.summary')}</p>
	</div>
	<div class="features__grid">
		{#each featureKeys as feature}
			<article class="feature-card">
				<h3>{$t(feature.title)}</h3>
				<p>{$t(feature.description)}</p>
			</article>
		{/each}
	</div>
</section>

<section class="catalog" id="catalog" aria-labelledby="catalog-heading">
	<div class="section-heading">
		<h2 id="catalog-heading">{$t('home.categories.heading')}</h2>
		<p>{$t('home.categories.summary')}</p>
	</div>
	<div class="catalog__grid">
		{#each categoryKeys as category}
			<article class="category-card">
				<h3>{$t(category.title)}</h3>
				<p>{$t(category.description)}</p>
			</article>
		{/each}
	</div>
</section>

<section class="cta-ribbon" aria-labelledby="cta-heading">
	<div class="cta-ribbon__content">
		<h2 id="cta-heading">{$t('home.ribbon.heading')}</h2>
		<p>{$t('home.ribbon.description')}</p>
	</div>
	<a class="cta-ribbon__button" href={data.loginUrl}>{$t('home.ribbon.cta')}</a>
</section>

<section class="faq" aria-labelledby="faq-heading">
	<div class="section-heading">
		<h2 id="faq-heading">{$t('home.faq.heading')}</h2>
		<p>{$t('home.faq.summary')}</p>
	</div>
	<div class="faq__grid">
		{#each faqKeys as faq}
			<details class="faq__item">
				<summary>{$t(faq.question)}</summary>
				<p>{$t(faq.answer)}</p>
			</details>
		{/each}
	</div>
</section>

<section class="support" aria-labelledby="support-heading">
	<div class="support__content">
		<h2 id="support-heading">{$t('home.support.heading')}</h2>
		<p>{$t('home.support.description')}</p>
	</div>
	<ul class="support__list">
		<li>
			<strong>{$t('home.support.items.verification.title')}</strong>
			<span>{$t('home.support.items.verification.description')}</span>
		</li>
		<li>
			<strong>{$t('home.support.items.guides.title')}</strong>
			<span>{$t('home.support.items.guides.description')}</span>
		</li>
		<li>
			<strong>{$t('home.support.items.backups.title')}</strong>
			<span>{$t('home.support.items.backups.description')}</span>
		</li>
	</ul>
</section>

<style>
	:global(body) {
		background-color: #0f172a;
		color: #e2e8f0;
		font-family: 'Inter', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
	}

	section {
		padding: clamp(3rem, 8vw, 6rem) clamp(1.5rem, 6vw, 5rem);
	}

	.section-heading {
		max-width: 720px;
		margin: 0 auto 2.5rem;
		text-align: center;
	}

	.section-heading h2 {
		font-size: clamp(1.8rem, 4vw, 2.6rem);
		margin-bottom: 0.75rem;
	}

	.section-heading p {
		color: rgba(226, 232, 240, 0.8);
	}

	.hero {
		padding-top: clamp(4rem, 10vw, 7rem);
	}

	.hero__content {
		max-width: 900px;
		margin: 0 auto;
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.hero__eyebrow {
		font-size: 0.9rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.2em;
		color: rgba(124, 58, 237, 0.8);
	}

	.hero h1 {
		font-size: clamp(2.5rem, 6vw, 3.8rem);
		line-height: 1.1;
	}

	.hero__description {
		font-size: clamp(1rem, 2.5vw, 1.2rem);
		color: rgba(226, 232, 240, 0.85);
		max-width: 720px;
		margin: 0 auto;
	}

	.hero__actions {
		display: flex;
		justify-content: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.hero__cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.9rem 1.75rem;
		border-radius: 9999px;
		font-weight: 600;
		text-decoration: none;
		transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
		border: 1px solid transparent;
	}

	.hero__cta--primary {
		background: linear-gradient(135deg, #7c3aed, #2563eb);
		color: #f8fafc;
		box-shadow: 0 16px 40px rgba(124, 58, 237, 0.25);
	}

	.hero__cta--primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 20px 50px rgba(37, 99, 235, 0.3);
	}

	.hero__cta--secondary {
		color: #cbd5f5;
		border-color: rgba(148, 163, 184, 0.4);
		background: rgba(148, 163, 184, 0.1);
	}

	.hero__cta--secondary:hover {
		background: rgba(148, 163, 184, 0.2);
		transform: translateY(-1px);
	}

	.hero__trust {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
		gap: 1rem;
		margin-top: 1.5rem;
		padding: 0;
		list-style: none;
		font-size: 0.95rem;
		color: rgba(148, 163, 184, 0.85);
	}

	.hero__trust li {
		padding: 0.75rem;
		border-radius: 0.75rem;
		border: 1px solid rgba(148, 163, 184, 0.15);
		background: rgba(15, 23, 42, 0.5);
	}

	.features__grid,
	.catalog__grid {
		display: grid;
		gap: 1.5rem;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
	}

	.feature-card,
	.category-card {
		padding: 1.75rem;
		border-radius: 1.25rem;
		background: rgba(15, 23, 42, 0.65);
		border: 1px solid rgba(148, 163, 184, 0.15);
		box-shadow: inset 0 1px 0 rgba(148, 163, 184, 0.05);
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.feature-card h3,
	.category-card h3 {
		font-size: 1.2rem;
	}

	.feature-card p,
	.category-card p {
		color: rgba(226, 232, 240, 0.75);
	}

	.cta-ribbon {
		margin: 0 clamp(1rem, 4vw, 3rem);
		background: linear-gradient(120deg, rgba(124, 58, 237, 0.2), rgba(37, 99, 235, 0.2));
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 1.5rem;
		padding: clamp(2rem, 5vw, 3rem);
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1.5rem;
	}

	.cta-ribbon__content h2 {
		font-size: clamp(1.5rem, 4vw, 2.2rem);
		margin-bottom: 0.5rem;
	}

	.cta-ribbon__content p {
		color: rgba(226, 232, 240, 0.8);
		max-width: 540px;
	}

	.cta-ribbon__button {
		background: #f8fafc;
		color: #0f172a;
		padding: 0.85rem 1.8rem;
		border-radius: 9999px;
		font-weight: 600;
		text-decoration: none;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.cta-ribbon__button:hover {
		transform: translateY(-2px);
		box-shadow: 0 12px 30px rgba(248, 250, 252, 0.25);
	}

	.faq__grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
		gap: 1.25rem;
	}

	.faq__item {
		background: rgba(15, 23, 42, 0.65);
		border: 1px solid rgba(148, 163, 184, 0.15);
		padding: 1.25rem 1.5rem;
		border-radius: 1rem;
	}

	.faq__item summary {
		cursor: pointer;
		font-weight: 600;
		outline: none;
	}

	.faq__item summary::-webkit-details-marker {
		display: none;
	}

	.faq__item summary::after {
		content: '＋';
		float: right;
		font-weight: 400;
	}

	.faq__item[open] summary::after {
		content: '－';
	}

	.faq__item p {
		margin-top: 1rem;
		color: rgba(226, 232, 240, 0.75);
	}

	.support {
		display: grid;
		gap: 2rem;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		align-items: center;
	}

	.support__content h2 {
		font-size: clamp(1.6rem, 4vw, 2.4rem);
		margin-bottom: 0.5rem;
	}

	.support__content p {
		color: rgba(226, 232, 240, 0.75);
		max-width: 520px;
	}

	.support__list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: grid;
		gap: 1rem;
	}

	.support__list li {
		background: rgba(15, 23, 42, 0.65);
		border: 1px solid rgba(148, 163, 184, 0.15);
		padding: 1.25rem;
		border-radius: 1rem;
		display: grid;
		gap: 0.35rem;
	}

	.support__list strong {
		font-size: 1.05rem;
	}

	.support__list span {
		color: rgba(226, 232, 240, 0.75);
	}

	@media (max-width: 640px) {
		.hero__trust {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.cta-ribbon {
			margin: 0;
		}
	}
</style>


