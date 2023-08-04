<script lang="ts">
  import { encrypt_message } from "rage-webassembly";
  import { publicKeyStore } from "$lib/public_key_store";
  import PublicKeyPicker from "$lib/components/PublicKeyPicker.svelte";
  import CopyToClipboard from "$lib/components/CopyToClipboard.svelte";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { get } from "svelte/store";

  let cipherText: string | null = null;
  let error: string | null = null;

  let plainText = "";

  let selection: string[] = [];

  const handleSubmit = () => {
    error = null;
    cipherText = null;
    const publicKeys = selection.flatMap((s) => get(publicKeyStore)[s]);
    try {
      cipherText = encrypt_message(plainText, publicKeys);
    } catch (e: unknown) {
      error = String(e);
    }
  };
</script>

{#if Object.keys($publicKeyStore).length == 0}
  <p>No public keys available. Please import theme <a href="/public-keys">here</a>.</p>
{:else}
  <form on:submit|preventDefault={handleSubmit}>
    <PublicKeyPicker bind:selection />
    <textarea required rows={10} placeholder="Ciphertext" bind:value={plainText} />
    <button>Encrypt</button>
  </form>
{/if}

{#if cipherText != null}
  <article>
    <h2>Ciphertext</h2>
    <pre>{cipherText}</pre>

    <p>
      <a href={`/decrypt-hash#${encodeURIComponent(cipherText)}`}>Link to decrypt</a>
    </p>

    <CopyToClipboard text={cipherText} label="Copy text to clipboard" />
    <CopyToClipboard
      text={`${window.location.origin}/decrypt-hash#${encodeURIComponent(cipherText)}`}
      label="Copy link to clipboard"
    />
  </article>
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
