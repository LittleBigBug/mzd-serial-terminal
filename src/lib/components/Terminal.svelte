<script>
	import click from "$lib/actions/click";
	import { sleep } from "$lib/util";
	import { writable } from "svelte/store";

	/**
	 * @type {string}
	 */
	export let logs;
	/**
	 * @type {string}
	 */
	export let input = "";
	export let allowInput = false;

	/**
	 * @type {HTMLElement}
	 */
	let termElement;
	/**
	 * @type {HTMLElement}
	 */
	let inputElement;

	let doEvent = true;
	let inputChange = false;

	const yScroll = writable(0);
	const focused = writable(false);
	const autoScrollPaused = writable(false);

	const focusIn = () => focused.set(true);
	const focusOut = () => focused.set(false);

	const inputChanged = () => {
		inputChange = true;
		sleep(100).then(() => (inputChange = false));
	};

	const heightChanged = () => {
		if ($autoScrollPaused) return;
		yScroll.set(termElement.scrollHeight - termElement.clientHeight);
	};

	const scroll = () => {
		doEvent = false;
		(termElement.scrollTop = $yScroll);

		// todo; dumb hack? pls fix
		sleep(100).then(() => (doEvent = true));
	};

	// todo; why the fuck aren't reactive statements working with this store?
	// $: termElement && scroll($yScroll);
	// $: console.log($yStore, "?????????");
	yScroll.subscribe((a) => termElement && scroll());

	$: input && inputChanged();
	$: termElement && (logs || input || !$autoScrollPaused) && heightChanged();
</script>

<div
	class="terminal"
	bind:this={termElement}
	use:click={() => inputElement?.focus()}
	on:scroll={() => doEvent && autoScrollPaused.set(true)}
>
	<span class="logs">{logs}</span>

	{#if allowInput}
		<span class="input">{input}</span>
		<span
			class="input-blinker"
			class:inputChange
			class:focused={$focused}
		/>
		<input
			type="text"
			on:focusin={focusIn}
			on:focusout={focusOut}
			bind:this={inputElement}
			bind:value={input}
		/>
	{/if}

	{#if $autoScrollPaused}
		<div
			class="scroll-paused"
			use:click={() => autoScrollPaused.set(false)}
		>
			Auto-scroll is paused. Click to resume.
		</div>
	{/if}
</div>

<style lang="scss">
	@keyframes blinker {
		0% {
			opacity: 1;
		}
		100% {
			opacity: 0;
		}
	}

	.terminal {
		color: white;
		font-size: 0;
		background: black;
		max-height: 100%;
		overflow-y: scroll;

		span, input {
			max-width: 100%;
			font-family: "JetBrains Mono", Monospaced, monospace;
			font-size: 14px;
		}

		span {
			display: inline;
			white-space: pre-wrap;
			word-wrap: break-word;
		}

		input {
			width: 0;
			opacity: 0;
			border: none;
			outline: none;
			background: none;
		}

		.input-blinker {
			width: 12px;
			height: 14px;
			background: white;
			display: inline-block;
			opacity: 0;

			&.inputChange {
				opacity: 1;
			}

			&.focused:not(.inputChange) {
				animation: blinker .9s ease-out infinite;
			}
		}

		.scroll-paused {
			width: 50%;
			opacity: 0.3;
			position: absolute;
			background-color: #23abaf;
		}
	}
</style>