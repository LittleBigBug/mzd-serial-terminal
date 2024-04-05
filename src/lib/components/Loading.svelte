<script>
	export let dots = 3;
</script>

<div class="loading">
	{#each Array(dots) as _, i (i)}
		<div
			class="dot"
			style:--offset={i}
		/>
	{/each}
</div>

<style lang="scss">
	@keyframes dot-blink {
		from {
			opacity: 1
		}
		to {
			opacity: 0.3
		}
	}

	.loading {
		gap: 4px;
		margin: 0 auto;
		display: flex;

		.dot {
			width: 6px;
			height: 6px;
			animation: dot-blink 1s infinite;
			animation-delay: calc(var(--offset) * 1.3s);
			border-radius: 20px;
			background-color: #ffffff;
		}
	}
</style>