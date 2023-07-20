<script lang="ts">
  import { encrypt_message } from "rage-webassembly";
  import ErrorMessage from "./ErrorMessage.svelte";
  import type { PublicKeyStore } from "$lib/public_key_store";

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

<form on:submit|preventDefault={handleSubmit}>
  <fieldset>
    {#each Object.keys(publicKeyStore) as recipientName}
      <label for={`recipient-${recipientName}`}>
        <input
          type="checkbox"
          bind:group={selection}
          id={`recipient-${recipientName}`}
          name={recipientName}
          value={recipientName}
        />
        {recipientName}
      </label>
    {/each}
  </fieldset>
  <textarea required rows={10} placeholder="Ciphertext" bind:value={plainText} />
  <button>Encrypt</button>
</form>

{#if cipherText != null}
  <article>
    <h2>Ciphertext</h2>
    <pre>{cipherText}</pre>
  </article>
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
