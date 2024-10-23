<script lang="ts">
  import { preventDefault } from "svelte/legacy";

  import { publicKeyStore } from "$lib/public_key_store";
  interface Props {
    recipientName: string;
  }

  let { recipientName }: Props = $props();

  let publicKeyToAdd = $state("");

  function addKey() {
    publicKeyStore.update((current) => {
      const keys = current[recipientName] ?? [];
      return {
        ...current,
        [recipientName]: [...keys, publicKeyToAdd],
      };
    });
    publicKeyToAdd = "";
  }

  function deleteRecipient() {
    publicKeyStore.update((current) => {
      const newKeys = { ...current };
      delete newKeys[recipientName];
      return newKeys;
    });
  }

  function deletePublicKey(publicKey: string) {
    publicKeyStore.update((current) => {
      const keys = current[recipientName] ?? [];

      const newKeys = keys.filter((key) => key != publicKey);

      return {
        ...current,
        [recipientName]: newKeys,
      };
    });
  }
</script>

{#if $publicKeyStore[recipientName]}
  <article>
    <header>{recipientName}</header>
    {#each $publicKeyStore[recipientName] as key}
      <div class="key">
        <pre>{key}</pre>
        <button onclick={() => deletePublicKey(key)} type="button">Delete</button>
      </div>
    {/each}

    <footer>
      <form onsubmit={preventDefault(addKey)} class="add-key-form">
        <input name="public_key" required placeholder="Public key" bind:value={publicKeyToAdd} />
        <button type="submit">add</button>
      </form>
      <button type="button" onclick={deleteRecipient}>Delete Recipient</button>
    </footer>
  </article>
{/if}

<style lang="scss">
  .add-key-form {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 10px;
  }
  .key {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 10px;
  }
</style>
