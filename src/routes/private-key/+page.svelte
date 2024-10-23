<script lang="ts">
  import PrivateKeyCard from "$lib/components/PrivateKeyCard.svelte";
  import { privateKeyStore } from "$lib/private_key_store";
  import type { FormEventHandler } from "svelte/elements";

  let keyName: string = $state("");
  let keyContent: string = $state("");

  const handleSubmit: FormEventHandler<HTMLFormElement> = (ev) => {
    ev.preventDefault();
    privateKeyStore.update((current) => ({
      ...current,
      [keyName]: keyContent,
    }));

    ev.currentTarget.reset();
  };

  const handleDelete = (keyName: string) => {
    privateKeyStore.update((current) => {
      const newStore = { ...current };
      delete newStore[keyName];
      return newStore;
    });
  };
</script>

<form onsubmit={handleSubmit}>
  <input required name="key_name" placeholder="Name of your age key" bind:value={keyName} />
  <textarea required placeholder="Private key" bind:value={keyContent}></textarea>
  <button>Add Key</button>
</form>
{#each Object.keys($privateKeyStore) as name}
  <PrivateKeyCard {name} privateKey={$privateKeyStore[name]} {handleDelete} />
{/each}
