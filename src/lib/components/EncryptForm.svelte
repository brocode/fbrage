<script lang="ts">
  import { encrypt_message } from "rage-webassembly";
  import ErrorMessage from "./ErrorMessage.svelte";
  import type { PublicKeyStore } from "$lib/public_key_store";
  import PublicKeyPicker from "./PublicKeyPicker.svelte";

  let cipherText: string | null = null;
  let error: string | null = null;

  export let publicKeyStore: PublicKeyStore;

  let plainText = "";

  let selection: string[] = [];

  const handleSubmit = () => {
    error = null;
    cipherText = null;
    const publicKeys = selection.flatMap((s) => publicKeyStore[s]);
    try {
      cipherText = encrypt_message(plainText, publicKeys);
    } catch (e: unknown) {
      error = String(e);
    }
  };
</script>

{#if Object.keys(publicKeyStore).length == 0}
  <p>No public keys available. Please import theme <a href="/public-keys">here</a>.</p>
{:else}
  <form on:submit|preventDefault={handleSubmit}>
    <PublicKeyPicker bind:selection {publicKeyStore} />
    <textarea required rows={10} placeholder="Ciphertext" bind:value={plainText} />
    <button>Encrypt</button>
  </form>
{/if}

{#if cipherText != null}
  <article>
    <h2>Ciphertext</h2>
    <pre>{cipherText}</pre>

    <a href={`/decrypt-hash#${encodeURIComponent(cipherText)}`}>Link to decrypt</a>
  </article>
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
