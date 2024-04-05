<script>
	import click from "$lib/actions/click";
	import { currentModal, modalComponent, modalData, modalName } from "$lib/stores/modal";

	const close = () => currentModal.set(undefined);
</script>

{#if $currentModal}
	<div class="backdrop" use:click={() => $modalData?.canClose && close()} />
	<div class="modal" use:click={(event) => event.stopPropagation()}>
		<div class="title">{$modalName}</div>
		<div class="content">
			{#if $modalData?.canClose}
				<div class="close-modal link" use:click={close}>x</div>
			{/if}

			<svelte:component this={$modalComponent} data={$modalData} />
		</div>
	</div>
{/if}

<style lang="scss">
	.backdrop {
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		backdrop-filter: blur(8px);
		position: absolute;
	}

	.modal {
		top: 50%;
		left: 50%;
		width: 80%;
		z-index: 10;
		color: white;
		position: absolute;
		border-radius: 12px;
		background-color: #424242;
		transform: translate(-50%, -50%);
		font-family: "JetBrains Mono", Monospaced, monospace;

		.title {
			padding: 12px;
			background-color: #323232;
			border-top-left-radius: 12px;
			border-top-right-radius: 12px;
			font-weight: bolder;
			font-size: 22px;
		}

		.content {
			padding: 12px;
			display: flex;
			flex-direction: column;
		}
	}

	.close-modal {
		position: absolute;
		top: 5px;
		right: 15px;
		font-weight: 700;
		color: var(--t-altTextColor);
	}
</style>