<script lang="ts">
	export let type: 'text' | 'password' = 'text';
	export let placeholder: string = '';
	export let id: string;
	export let label: string = '';
	export let error: string = '';
	export let value: string = '';
	export let valid: boolean = true;
	export let validator: ((value: string) => string) | undefined = undefined;

	let interacted = false;

	function on_input(e: Event) {
		//@ts-ignore
		value = e.target.value;
		interacted = true;
	}
</script>

<div>
	{#if label}
		<label for={id} class="main-label">{label}</label>
	{/if}
	<div class="text-box flex">
		<div class="left-prop">
			{#if $$slots.left}
				<slot name="left" />
			{/if}
		</div>
		<input {type} {id} {placeholder} name="input" class="input px-3 py-2" on:input={on_input} />
		{#if $$slots.right}
			<div class="right-prop">
				<slot name="right" />
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	.main-label {
		@apply block text-sm font-medium text-gray-700;
	}

	.text-box {
		@apply border-gray-300 border mt-1 relative rounded-md shadow-sm w-full;
	}

	.text-box:focus-within {
		@apply border-indigo-500 ring-indigo-500;
	}

	.left-prop {
		@apply inset-y-0 left-0 pl-3 flex items-center;
	}

	.right-prop {
		@apply right-0 flex items-center;
	}

	.input {
		@apply block sm:text-sm rounded-md flex-1 focus:outline-none;
	}
</style>
