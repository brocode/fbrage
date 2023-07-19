import type { Writable, Unsubscriber, Updater } from "svelte/store";
import { writable } from "svelte/store";

interface Storable<T> {
  subscribe: (run: (value: T) => void, invalidate?: (value?: T) => void) => Unsubscriber;
  set: (value: T) => void;
  update: (updater: Updater<T>) => void;
}

export function storable<T>(storageKey: string, initialData: T): Storable<T> {
  const store: Writable<T> = writable(initialData);
  const { subscribe, set, update } = store;

  if (localStorage.getItem(storageKey)) {
    set(JSON.parse(localStorage.getItem(storageKey) as string));
  }

  return {
    subscribe,
    set: (n: T) => {
      localStorage.setItem(storageKey, JSON.stringify(n));
      set(n);
    },
    update: (cb: (value: T) => T) => {
      update((n: T) => {
        const updated = cb(n);
        localStorage.setItem(storageKey, JSON.stringify(updated));
        return updated;
      });
    },
  };
}
