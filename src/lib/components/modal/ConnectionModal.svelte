<script>
	import click from "$lib/actions/click";
	import { portDiff, portPath } from "$lib/stores/serial";
	import { openModal, closeModal } from "$lib/stores/modal";

	import Await from "$lib/components/Await.svelte";
	import Loading from "$lib/components/Loading.svelte";

	const rx = / ?\(?COM\d+\)? ?/i;

	const getPorts = async () => {
		const detected = await portPath.detect();
		const suggestedPorts = detected?.suggested_ports
			?.map((port) => ({
				...port,
				suggested: true,
			})) || [];
		const allPorts = detected?.all_ports
				?.filter((port) => suggestedPorts.find((sPort) => sPort.name !== port.name))
			|| [];

		return suggestedPorts
			.concat(allPorts)
			.map((port) => ({
				...port,
				product_name: port?.product_name?.replace(rx, ""),
			}));
	};

	/**
	 * @param {string} path
	 */
	const connect = (path) => {
		portPath.set(path).then(closeModal);
		openModal(`Connecting to ${path}`, Loading);
	};

	/**
	 * @type {Promise<any>}
	 */
	let promise;

	const fetch = () => (promise = getPorts());

	fetch();

	$: $portDiff && fetch();
</script>

<Await {promise} let:data={ports}>
	{#if !ports?.length}
		<span class="found-ports">No ports found. Is your device properly connected?</span>
	{:else}
		<span class="found-ports">Found port(s):</span>

		{#each ports as { name, product_name, suggested }, i (i)}
			{#if suggested && i === 0}
				<span class="port-title">Suggested</span>
			{:else if !suggested && (i === 0 || ports[i - 1].suggested)}
				<span class="port-title">Other Ports</span>
			{/if}

			<div
				class="port"
				class:first={i === 0}
				class:last={i === (ports.length - 1)}
			>
				<span class="path">{name}</span>
				<span class="name">{product_name}</span>
				<span
					class="connect"
					use:click={() => connect(name)}
				>
					Connect
				</span>
			</div>
		{/each}
	{/if}

	<div class="loading" slot="loading">
		<span class="found-ports">Finding Ports</span>
		<Loading />
	</div>

	<span class="footer-note">
		<Loading dots={1} />
		Watching for port connections
	</span>
</Await>

<style lang="scss">
	.found-ports, .port-title {
		display: block;
		font-weight: bold;
	}
	.found-ports {
		font-size: 18px;
	}
	.port-title {
		padding-top: 12px;
		padding-bottom: 12px;
	}
	.port {
		display: flex;
		justify-content: space-between;
		background-color: #323232;

		&.first {
			border-top-left-radius: 12px;
			border-top-right-radius: 12px;

			.path {
				border-top-left-radius: 12px;
			}
			.connect {
				border-top-right-radius: 12px;
			}
		}
		&.last {
			border-bottom-left-radius: 12px;
			border-bottom-right-radius: 12px;

			.path {
				border-bottom-left-radius: 12px;
			}
			.connect {
				border-bottom-right-radius: 12px;
			}
		}

		span {
			padding: 15px;
		}
		.path {
			background: #292929;
		}
		.connect {
			background: #39733a;

			&:hover {
				cursor: pointer;
				background: #275428;
			}
		}
	}
	.loading {
		gap: 8px;
		display: flex;
		flex-direction: column;
	}
	.footer-note {
		margin-top: 10px;
		font-size: 10px;

		:global(.loading) {
			display: inline-flex;
		}
	}
</style>