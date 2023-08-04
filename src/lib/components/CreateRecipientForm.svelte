<script lang="ts">
  import { publicKeyStore } from "$lib/public_key_store";

  let recipientName = "";

  function createRecipient() {
    publicKeyStore.update((current) => {
      return {
        ...current,
        [recipientName]: [],
      };
    });
    recipientName = "";
  }
</script>

<article>
  <header>Create recipient</header>

  <form on:submit|preventDefault={createRecipient}>
    <input name="recipient_name" required placeholder="Recipient name" bind:value={recipientName} />
    <button type="submit">Create</button>
  </form>
</article>
