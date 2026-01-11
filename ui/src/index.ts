import { mount } from "svelte";
import App from "./App.svelte";

// @ts-expect-error
const app = mount(App, {
  target: document.body,
});
