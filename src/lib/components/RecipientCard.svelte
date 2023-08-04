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
</script>

<article>
  <header>{recipientName}</header>
  {#each $publicKeyStore[recipientName] as key}
    <pre>{key}</pre>
  {/each}

  <button type="button" on:click={deleteRecipient}>Delete Recipient</button>
</article>
