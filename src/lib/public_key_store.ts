import { derived } from "svelte/store";
import { storable } from "./storage";

export type PublicKeyStore = Record<string, string[]>;

export const publicKeyStore = storable<PublicKeyStore>("agePublicKeys", {});

export const sortedKeys = derived(publicKeyStore, (store) => {
  return Object.keys(store).sort();
});
