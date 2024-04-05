<script>
	import Loading from "$lib/components/Loading.svelte";

	/**
	 * @type {Promise<any>}
	 */
	export let promise;
</script>

{#if !promise}
	<slot name="loading">
		<Loading />
	</slot>
{:else}
	{#await promise}
		<slot name="loading">
			<Loading />
		</slot>
	{:then data}
		<slot {data} />
	{:catch error}
		<slot name="error">
			<p style="color: red">{error.message}</p>
		</slot>
	{/await}
{/if}