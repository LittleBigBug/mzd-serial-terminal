import { derived, writable } from "svelte/store";

/**
 * @typedef {Object} Modal
 * @property {string} name
 * @property {any} component
 * @property {Object} data
 */

/**
 * @type {Writable<Modal>}
 */
export const currentModal = writable();
export const modalName = derived([currentModal], ([$modal]) => $modal?.name);
export const modalComponent = derived([currentModal], ([$modal]) => $modal?.component);
export const modalData = derived([currentModal], ([$modal]) => $modal?.data);

export const openModal = (name, component, data) => currentModal.set({ name, component, data });
export const closeModal = () => currentModal.set(undefined);