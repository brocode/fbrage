<script lang="ts">
  import { get_public_key_for_private_key } from "rage-webassembly";
  import ErrorMessage from "./ErrorMessage.svelte";

  export let name: string;
  export let privateKey: string;
  export let handleDelete: (name: string) => void;
  let publicKey: string | null = null;
  let error: string | null = null;

  function getPublicKeyForPrivateKey(key: string) {
    error = null;
    publicKey = null;
    try {
      publicKey = get_public_key_for_private_key(key);
    } catch (e) {
      error = String(e);
    }
  }

  $: getPublicKeyForPrivateKey(privateKey);
</script>

<article>
  <h1>{name}</h1>
  {#if publicKey != null}
    <pre>{publicKey}</pre>
  {/if}
  {#if error != null}
    <ErrorMessage {error} />
  {/if}
  <button on:click={() => handleDelete(name)}>Delete</button>
</article>
