<script lang="ts">
	import { logout } from '$lib/requests';
	import { token } from '$lib/stores';
	import Sessions from './sessions.svelte';

	let tab: 0 | 1 = 0;

	function logout_click() {
		logout().finally(() => token.set(undefined));
	}
</script>

<nav class="appbar">
	<div class="appbar-contents">
		<div class="bar-left">
			<h1 class="bar-title">ImageMatch</h1>

			{#if $token?.admin}
				<button on:click={() => (tab = 0)} class="tab-button">Galerie</button>
				<button on:click={() => (tab = 1)} class="tab-button">Administration</button>
			{/if}
		</div>
		<button on:click={logout_click} class="disconnect">Déconnexion</button>
	</div>
</nav>
<div class="tab-container">
	{#if tab === 0}
		<Sessions />
	{:else}
		<h1>Administration</h1>
	{/if}
</div>

<style lang="postcss">
	.appbar {
		@apply drop-shadow bg-white;
	}

	.appbar-contents {
		@apply max-w-7xl mx-auto px-2 sm:px-6 lg:px-8 relative flex items-center justify-between h-16;
	}

	.bar-left {
		@apply flex-1 flex items-center justify-center sm:justify-start space-x-2;
	}

	.bar-title {
		@apply text-lg font-semibold;
	}

	.bar-button {
		@apply px-3 py-2 rounded-md font-medium;
	}

	.tab-button {
		@apply bar-button text-blue-900 hover:bg-blue-900 hover:text-white;
	}

	.disconnect {
		@apply bar-button text-red-600 hover:bg-red-300 float-left;
	}

	.tab-container {
		@apply mx-4 my-3;
	}
</style>
