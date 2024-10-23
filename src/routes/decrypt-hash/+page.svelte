<script lang="ts">
  import { decrypt_message } from "rage-webassembly";
  import { privateKeyStore } from "$lib/private_key_store";
  import { get } from "svelte/store";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import Message from "$lib/components/Message.svelte";

  let hash = decodeURIComponent(window.location.hash.substring(1));
  let error: string | null = $state(null);

  let decryptedMessage: string | null = $state(null);
  try {
    let privateKeys = Object.values(get(privateKeyStore));
    decryptedMessage = decrypt_message(hash, privateKeys);
  } catch (e: unknown) {
    error = String(e);
  }
</script>

{#if decryptedMessage != null}
  <Message message={decryptedMessage} />
{/if}
{#if error != null}
  <ErrorMessage {error} />
{/if}
