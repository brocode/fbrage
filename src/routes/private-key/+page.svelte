<script lang="ts">
  import { privateKeyStore } from "$lib/private_key_store";
  import type { FormEventHandler } from "svelte/elements";

  let keyName: string;
  let keyContent: string;

  const handleSubmit: FormEventHandler<HTMLFormElement> = (ev) => {
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

<form on:submit|preventDefault={handleSubmit}>
  <input name="key_name" placeholder="Name of your age key" bind:value={keyName} />
  <textarea placeholder="Private key" bind:value={keyContent} />
  <button>Add Key</button>
</form>
{#each Object.keys($privateKeyStore) as name}
  <article>
    <h1>{name}</h1>
    <button on:click={() => handleDelete(name)}>Delete</button>
  </article>
{/each}
