<script lang="ts">
  import { run } from "svelte/legacy";

  import { get_public_key_for_private_key } from "rage-webassembly";
  import ErrorMessage from "./ErrorMessage.svelte";

  interface Props {
    name: string;
    privateKey: string;
    handleDelete: (name: string) => void;
  }

  let { name, privateKey, handleDelete }: Props = $props();
  let publicKey: string | null = $state(null);
  let error: string | null = $state(null);

  function getPublicKeyForPrivateKey(key: string) {
    error = null;
    publicKey = null;
    try {
      publicKey = get_public_key_for_private_key(key);
    } catch (e) {
      error = String(e);
    }
  }

  run(() => {
    getPublicKeyForPrivateKey(privateKey);
  });
</script>

<article>
  <h1>{name}</h1>
  {#if publicKey != null}
    <pre>{publicKey}</pre>
  {/if}
  {#if error != null}
    <ErrorMessage {error} />
  {/if}
  <button onclick={() => handleDelete(name)}>Delete</button>
</article>
