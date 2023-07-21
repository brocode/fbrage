<script lang="ts">
  import { genkey } from "rage-webassembly";
  import { privateKeyStore } from "$lib/private_key_store";
  import { get } from "svelte/store";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";

  let nameInput: HTMLInputElement | null = null;
  let keyName = "";

  function checkIfKeyExists(keyName: string) {
    if (keyName && get(privateKeyStore)[keyName]) {
      nameInput?.setCustomValidity("Key already exists");
    } else {
      nameInput?.setCustomValidity("");
    }
  }

  $: checkIfKeyExists(keyName);

  let error: string | null = null;

  type GeneratedKey = { public_key: string; private_key: string };
  let generatedKey: GeneratedKey | null = null;

  function handleGenKey() {
    error = null;
    try {
      let gen: GeneratedKey = genkey();
      generatedKey = gen;
      privateKeyStore.update((keys) => {
        return {
          ...keys,
          [keyName]: gen.private_key,
        };
      });
    } catch (e) {
      error = String(e);
    }
  }
</script>

{#if generatedKey == null}
  <p>The key will automatically be stored in your private keys. You need to publish your public key yourself.</p>

  <form on:submit|preventDefault={handleGenKey}>
    <input bind:this={nameInput} name="key_name" required placeholder="Name of your age key" bind:value={keyName} />
    <button>Generate key</button>
  </form>
{:else}
  <article>
    <h2>Generated key</h2>
    <p>This is the last time this is shown to you!</p>
    <h3>Public key</h3>
    <pre>{generatedKey.public_key}</pre>
    <h3>Private key</h3>
    <pre>{generatedKey.private_key}</pre>
  </article>
{/if}

{#if error != null}
  <ErrorMessage {error} />
{/if}
