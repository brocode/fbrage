<script lang="ts">
  import type { PrivateKeyStore } from "$lib/private_key_store";
  import { decrypt_message } from "rage-webassembly";
  import ErrorMessage from "./ErrorMessage.svelte";

  let plainText: string | null = null;
  let error: string | null = null;

  export let privateKeyStore: PrivateKeyStore;

  let cipherText = "";

  const handleSubmit = () => {
    error = null;
    plainText = null;
    try {
      plainText = decrypt_message(cipherText, Object.values(privateKeyStore));
    } catch (e: unknown) {
      error = String(e);
    }
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <textarea required rows={10} placeholder="Ciphertext" bind:value={cipherText} />
  <button>Decrypt</button>
</form>

{#if plainText != null}
  <article>
    <h2>Plaintext</h2>
    <div>
      {plainText}
    </div>
  </article>
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
