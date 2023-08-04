<script lang="ts">
  import { publicKeyStore } from "$lib/public_key_store";
  export let recipientName: string;

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

<article>
  <header>{recipientName}</header>
  {#each $publicKeyStore[recipientName] as key}
    <div class="key">
      <pre>{key}</pre>
      <button on:click={() => deletePublicKey(key)} type="button">Delete</button>
    </div>
  {/each}

  <footer>
    <button type="button" on:click={deleteRecipient}>Delete Recipient</button>
  </footer>
</article>

<style lang="scss">
  .key {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 10px;
  }
</style>
