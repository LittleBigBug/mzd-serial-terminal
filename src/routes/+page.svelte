<script>
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";

	import { openModal } from "$lib/stores/modal";
    import { input, logLines, debugLines } from "$lib/stores/terminal";
	import { connected, portPath, portDiff } from "$lib/stores/serial";

    import Terminal from "$lib/components/Terminal.svelte";
    import ConnectionModal from "$lib/components/modal/ConnectionModal.svelte";

    let unlistenTerm;
    let unlistenDebug;
	let unlistenStatus;

	openModal("Connect to your Mazda", ConnectionModal);

    onMount(async () => {
        unlistenTerm = await listen("TerminalText", (e) => {
			//console.log(e?.payload?.text.replace(/\n/gi, '\\n'));
			logLines.add(e?.payload?.text);
        });

        unlistenDebug = await listen("DebugText", (e) => {
			debugLines.add(e?.payload?.text);
        });

		unlistenStatus = await listen("PortStatus", (e) => {
			// Port disconnected while we are connected to it
			if ($connected && e?.payload?.disconnected_port_paths?.includes($portPath))
				portPath.set();

			portDiff.set(e?.payload);
		});
    });
</script>

<div class="termWindows">
    <Terminal
            allowInput
            bind:input={$input}
            logs={$logLines}
    />
    <div class="resizer"></div>
    <Terminal logs={$debugLines} />
</div>

<style lang="scss">
    .termWindows {
      height: 100vh;
      display: flex;

      .resizer {
        min-width: .5%;
        background: grey;

        &:hover {
          cursor: ew-resize;
        }
      }
      :global(.terminal) {
        max-width: 49.75%;
        flex-basis: 49.75%;
      }
    }
</style>