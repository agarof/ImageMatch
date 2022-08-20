<script context="module" lang="ts">
	import { token } from '$lib/stores';
	import Login from '$lib/../routes/login.svelte';
	import App from './app.svelte';
</script>

{#if $token === undefined}
	<Login />
{:else}
	<App />
{/if}
