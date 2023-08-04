<script lang="ts">
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import Message from "$lib/components/Message.svelte";
  import { privateKeyStore } from "$lib/private_key_store";
  import { decrypt_message } from "rage-webassembly";
  import { get } from "svelte/store";

  let plainText: string | null = null;
  let error: string | null = null;

  let cipherText = "";

  const handleSubmit = () => {
    error = null;
    plainText = null;
    try {
      plainText = decrypt_message(cipherText, Object.values(get(privateKeyStore)));
    } catch (e: unknown) {
      error = String(e);
    }
  };
</script>

<p>Available private keys: {Object.keys($privateKeyStore).length}</p>

<form on:submit|preventDefault={handleSubmit}>
  <textarea required rows={10} placeholder="Ciphertext" bind:value={cipherText} />
  <button>Decrypt</button>
</form>

{#if plainText != null}
  <Message message={plainText} />
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
