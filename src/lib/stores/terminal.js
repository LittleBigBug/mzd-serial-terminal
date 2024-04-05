import { derived } from "svelte/store";
import { list, str } from "$lib/stores";

export const debugLines = str();
export const logLines = str();
export const history = list();

export const input = str();

export const command = (cmd) => {
    history.push(cmd);
    // todo; maybe better echo?
    logLines.add(`$> ${cmd}`);
};