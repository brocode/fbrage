<script lang="ts">
  import type { PrivateKeyStore } from "$lib/private_key_store";
  import { decrypt_message } from "rage-webassembly";

  let plainText: string | null = null;

  export let privateKeyStore: PrivateKeyStore;

  let cipherText = "";

  const handleSubmit = () => {
    plainText = decrypt_message(cipherText, Object.values(privateKeyStore));
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <textarea rows={10} placeholder="Ciphertext" bind:value={cipherText} />
  <button>Decrypt</button>
</form>

{#if plainText != null}
  <h2>Plaintext</h2>
  <div class="plaintext">
    {plainText}
  </div>
{/if}

<style lang="scss">
  .plaintext {
    border: var(--border-width) solid var(--form-element-border-color);
    padding: var(--form-element-spacing-vertical) var(--form-element-spacing-horizontal);
  }
</style>
