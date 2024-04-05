# Mazda Serial Terminal

Developer tool that connects a terminal to a CP210x USB UART bridge device that is wired to the back of your Mazda's CMU.

You can usually use something like Putty or SecureCRT to create a terminal, but the Mazda sends a crap-ton of debug
information along with the terminal's output.

This app splits the two into different terminal windows, allowing you to view (or disable) either, so you can see what
you're sending and receiving in the CLI.

This app is built with Svelte (JS) as the frontend, and Tauri (Rust) for the backend

## Developing

Before doing anything, you have to follow the [Tauri setup guide](https://tauri.app/v1/guides/getting-started/prerequisites#installing)
for your operating system.

After that, install npm dependencies, then you can run the app in dev mode:

```bash
npm ci
npm run dev
```