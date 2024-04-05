<script>
	import Modal from "$lib/components/modal/Modal.svelte";
</script>

<slot />
<Modal />

<style lang="scss">
    :global(body) {
      margin: 0;
      padding: 0;
      background: black;
    }
</style>