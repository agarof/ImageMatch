<script context="module" lang="ts">
	throw new Error("@migration task: Check code was safely removed (https://github.com/sveltejs/kit/discussions/5774#discussioncomment-3292722)");

	// import { token } from '$lib/stores';
	// import Login from '$lib/../routes/login.svelte';
	// import App from './app.svelte';
</script>

{#if $token === undefined}
	<Login />
{:else}
	<App />
{/if}
