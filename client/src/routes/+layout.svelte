<script lang="ts">
	import '../app.css';
	import type { LayoutData } from './$types';
	import type { SvelteSlots } from 'svelte';

	let { data, children } = $props<{ data: LayoutData; children: SvelteSlots }>();

	const site = data.site;
</script>

{#if site}
	<svelte:head>
		<meta name="application-name" content={site.name} />
		<meta name="apple-mobile-web-app-title" content={site.name} />
		<meta name="apple-mobile-web-app-capable" content="yes" />
		<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
		<meta property="og:site_name" content={site.name} />
		<meta name="theme-color" content="#0f172a" />
		<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
		<link rel="preconnect" href="https://fonts.googleapis.com" />
	</svelte:head>
{/if}

{@render children()}
