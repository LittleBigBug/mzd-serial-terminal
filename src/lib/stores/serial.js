import { invoke } from "@tauri-apps/api/tauri";
import { writable, derived } from "svelte/store";
import { openModal } from "$lib/stores/modal";
import { debugLines, logLines } from "$lib/stores/terminal";
import ConnectionModal from "$lib/components/modal/ConnectionModal.svelte";

/**
 * @typedef {Object} PortInfo
 * @property {string} name
 * @property {string} port_type
 * @property {string?} product_name
 * @property {string?} manufacturer
 * @property {boolean} suggested
 */

/**
 * @typedef {Object} PortsDetected
 * @property {PortInfo[]} all_ports - All ports found on the host device
 * @property {PortInfo[]} suggested_ports - All suggested/matching ports based on description
 */

let pathStore = writable();
export const portPath = {
    ...pathStore,
    /**
     * @param {string?} [path]
     * @returns {Promise<any>}
     */
    set(path) {
        pathStore.set(path);

        if (path && path !== "") {
            logLines.set("");
            debugLines.set("");

            return invoke("connect_serial", {path});
        } else {
            openModal("Connect to your Mazda", ConnectionModal);
            return invoke("close_serial");
        }
    },
    /**
     * Detect current COM serial ports
     * @returns {Promise<PortsDetected>}
     */
    detect() {
        return invoke("suggest_serial_ports");
    },
};

export const connected = derived([portPath], ([$path]) => !!$path);

export const portDiff = writable();