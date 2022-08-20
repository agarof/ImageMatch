<script lang="ts">
	import { login, register } from '$lib/requests';
	import { token } from '$lib/stores';
	import Input from '$lib/components/input.svelte';

	let login_mode = true;

	let email = '';
	let password = '';
	let password_confirmation = '';

	function text(login_mode: boolean): [string, string, string] {
		return login_mode
			? ["S'identifier", "Vous n'avez pas de compte ?", 'Inscrivez vous !']
			: ["S'inscrire", 'Vous avez déjà un compte ?', 'Identifiez vous !'];
	}

	function enabled(email: string, password: string, password_confirmation: string) {
		return (
			email.length !== 0 &&
			password.length !== 0 &&
			(login_mode || password_confirmation.length !== 0)
		);
	}

	function send() {
		if (enabled(email, password, password_confirmation)) {
			if (login_mode) {
				send_login();
			} else {
				send_register;
			}
		}
	}

	function send_login() {
		login({ email: email.trim(), password: password.trim() })
			.then((response) => token.set(response))
			.catch(console.log);
	}

	function send_register() {
		register({ email: email.trim(), password: password.trim() }).then(console.log);
	}
</script>

<div class="container">
	<form class="login-form" on:submit|preventDefault={send}>
		<h1 class="heading-text">{text(login_mode)[0]}</h1>

		<div>
			<Input type="text" id="username" placeholder="Email" bind:value={email} />
			<Input type="password" id="password" placeholder="Mot de passe" bind:value={password} />
			{#if !login_mode}
				<Input
					type="password"
					id="password"
					placeholder="Confirmation du mot de passe"
					bind:value={password_confirmation}
				/>
			{/if}
		</div>

		<button
			class="confirm-button"
			type="submit"
			disabled={!enabled(email, password, password_confirmation)}
		>
			{text(login_mode)[0]}
		</button>
		<p>{text(login_mode)[1]}</p>
		<button class="button " on:click={() => (login_mode = !login_mode)}>
			{text(login_mode)[2]}
		</button>
	</form>
</div>

<style lang="postcss">
	.container {
		@apply justify-center flex items-center h-screen mx-auto;
	}

	.login-form {
		@apply max-w-sm rounded-lg overflow-hidden shadow-lg px-6 py-4 space-y-4;
	}

	.heading-text {
		@apply mt-2 text-3xl font-extrabold text-gray-900 font-sans;
	}

	.button {
		@apply rounded-md border px-2 py-1 w-full disabled:bg-gray-200;
	}

	.confirm-button {
		@apply button bg-indigo-600 text-white hover:bg-indigo-700 disabled:bg-gray-200;
	}

	.test {
		@apply hidden;
	}
</style>
